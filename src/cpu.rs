use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use std::collections::VecDeque;
use hardware::Hardware;

// The 4004 is a 4 bit data / 12 bit address CPU therefore it doesn't really
// fit into the standard integer types. Comments below show actual size of the
// registers. Not sure how to best handle this.

const NUM_INDEX_REGISTERS: usize = 16;
const NUM_STACK_REGISTERS: usize = 3;

#[derive(Debug)]
pub struct CPU {               // actual register size
    accumulator: u8,           // u4
    carry: bool,               // u1

    program_counter: u16,      // u12

    program_counter_stack: VecDeque<u16>,

    index_registers: [u8; NUM_INDEX_REGISTERS], // u4

    // internal register used for ram bank switching
    command_control_register: u8,

    // internal registers for RAM/ROM/IO selection
    ram_address_register_0: u8, // sent at X2
    ram_address_register_1: u8, // sent at X3

    hardware: Hardware
}

impl CPU {
    pub fn new(hardware: Hardware) -> CPU {
        CPU {
            accumulator: 0,
            carry: false,
            program_counter: 0,
            program_counter_stack: VecDeque::with_capacity(NUM_STACK_REGISTERS),
            index_registers: [0; NUM_INDEX_REGISTERS],
            command_control_register: 0,
            ram_address_register_0: 0,
            ram_address_register_1: 0,
            hardware: hardware
        }
    }

    pub fn _reset(&mut self) {
        self.accumulator = 0;
        self.carry = false;
        self.program_counter = 0;
        self.program_counter_stack.clear();
        for x in 0..NUM_INDEX_REGISTERS {
            self.index_registers[x] = 0;
        }
        self.command_control_register = 0;
    }

    pub fn run(&mut self) {
        loop {
            println!("{}", self);
            self.run_instruction();
            sleep(Duration::from_millis(10));
        }
    }

    fn run_instruction(&mut self) {
        let (opr, opa) = self.rom_read_nibbles();

        match opr {
            0x0 => { }, // NOP What if opa != 0? Still a NOP?
            0x1 => self.opr_jcn(opa),
            0x2 => self.opr_src(opa),
            0x3 => match opa & 0b0001 {
                0 => self.opr_fin(opa),
                1 => self.opr_jin(opa),
                _ => panic!(), // remove compile error
            },
            0x4 => self.opr_jun(opa),
            0x5 => self.opr_jms(opa),
            0x7 => self.opr_isz(opa),
            0x8 => self.opr_add(opa),
            0x9 => self.opr_sub(opa),
            0xa => self.opr_ld(opa),
            0xb => self.opr_xch(opa),
            0xc => self.opr_bbl(opa),
            0xd => self.opr_ldm(opa),
            0xe => match opa {
                0x9 => self.opa_rdm(),
                _   => panic!("Unrecognized instruction: {:0x}{:0x}", opr, opa),
            },
            0xf => { // Accumulator Group Instructions
                match opa {
                    0x0 => self.opa_clb(),
                    0x1 => self.opa_clc(),
                    0x2 => self.opa_iac(),
                    0x3 => self.opa_cmc(),
                    0x4 => self.opa_cma(),
                    0x5 => self.opa_ral(),
                    0x6 => self.opa_rar(),
                    0x7 => self.opa_tcc(),
                    0x8 => self.opa_dac(),
                    0x9 => self.opa_tcs(),
                    0xa => self.opa_stc(),
                    0xb => self.opa_daa(),
                    0xc => self.opa_kbp(),
                    0xd => self.opa_dcl(),
                    _   => panic!("Unrecognized instruction: {:0x}{:0x}", opr, opa),
                }
            },
            _   => panic!("Unrecognized instruction: {:0x}{:0x}", opr, opa),
        }
    }

    fn ram_read_nibble(&self) -> u8 {
        let chip = self.ram_address_register_0 >> 2;
        let register = self.ram_address_register_0 & 0b0011;
        let character = self.ram_address_register_1;

        self.hardware.ram_read_char(chip, register, character)
    }

    fn rom_read_nibbles(&mut self) -> (u8, u8) {
        let byte = self.hardware.rom_read_byte(self.program_counter);
        self.program_counter += 1;

        ((byte >> 4) & 0b1111, byte & 0b1111)
    }

    fn program_counter_stack_push(&mut self) {
        if self.program_counter_stack.len() == NUM_STACK_REGISTERS {
            self.program_counter_stack.pop_back();
        }
        self.program_counter_stack.push_front(self.program_counter);
    }

    fn program_counter_stack_pop(&mut self) {
        self.program_counter = match self.program_counter_stack.pop_front() {
            Some(x) => x,
            None    => panic!("Program counter stack underflow.")
        };
    }

    // =========================V operands in order V=========================

    fn opr_jcn(&mut self, opa: u8) {
        let (a2, a1) = self.rom_read_nibbles();

        let invert_cond = opa & 0b1000 == 0b1000;
        let accumulator_cond = (self.accumulator == 0) && (opa & 0b0100 == 0b0100);
        let carry_cond  = (self.carry) && (opa & 0b0010 == 0b0010);
        let test_signal_cond = false && (opa & 0b0001 == 0b0001); // TODO implement

        let cond = accumulator_cond || carry_cond || test_signal_cond;

        if (!invert_cond && cond) || (invert_cond && !cond) {
            let ph = self.program_counter >> 8;
            self.program_counter = (ph << 8) + ((a2 as u16) << 4) + (a1 as u16);
        }
    }

    fn opr_src(&mut self, opa: u8) {
        self.ram_address_register_0 = self.index_registers[(opa & 0b1110) as usize];
        self.ram_address_register_1 = self.index_registers[(opa & 0b1111) as usize];
    }

    fn opr_fin(&mut self, opa: u8) {
        let (d2, d1) = self.rom_read_nibbles();
        // could make a write_register_pair() function.
        self.index_registers[opa as usize] = d2;
        self.index_registers[(opa + 1) as usize] = d1;
        self.program_counter += 1;
    }

    fn opr_jin(&mut self, opa: u8) {
        // we already incremented the pc so don't need to worry about the
        // ROM boundary as mentioned in the docs.
        let ph = self.program_counter >> 8;
        let pm = self.index_registers[(opa & 0b1110) as usize];
        let pl = self.index_registers[(opa & 0b1111) as usize];
        self.program_counter = (ph << 8) + ((pm as u16) << 4) + (pl as u16);
    }

    fn opr_jun(&mut self, opa: u8) {
        let (a2, a1) = self.rom_read_nibbles();
        self.program_counter = ((opa as u16) << 8)
                             + ((a2 as u16) << 4)
                             + a1 as u16;
    }

    fn opr_jms(&mut self, opa: u8) {
        let (a2, a1) = self.rom_read_nibbles();
        self.program_counter_stack_push();
        self.program_counter = ((opa as u16) << 8)
                             + ((a2 as u16) << 4)
                             + a1 as u16;
    }

    fn opr_isz(&mut self, opa: u8) {
        let (a2, a1) = self.rom_read_nibbles();
        self.index_registers[opa as usize] = (self.index_registers[opa as usize] + 1) % 16;

        if self.index_registers[opa as usize] != 0 {
            let ph = self.program_counter >> 8;
            self.program_counter = (ph << 8) + ((a2 as u16) << 4) + (a1 as u16);
        }
    }

    fn opr_add(&mut self, opa: u8) {
        let mut sum: u8 = self.index_registers[opa as usize] + self.accumulator;
        if self.carry { sum += 1; }
        self.accumulator = sum & 0b1111;
        if sum > 15 { self.carry = true; }
    }

    fn opr_sub(&mut self, opa: u8) {
        self.accumulator = self.accumulator + !self.index_registers[opa as usize];
        if !self.carry { self.accumulator += 1; }
        if self.accumulator > 15 { self.carry = true; }
        self.accumulator &= 0b1111;
    }

    fn opr_ld(&mut self, opa: u8) {
        self.accumulator = self.index_registers[opa as usize];
    }

    fn opr_xch(&mut self, opa: u8) {
        let tmp: u8 = self.accumulator;
        self.accumulator = self.index_registers[opa as usize];
        self.index_registers[opa as usize] = tmp;
    }

    fn opr_bbl(&mut self, opa: u8) {
        self.program_counter_stack_pop();
        self.accumulator = opa;
    }

    fn opr_ldm(&mut self, opa: u8) {
        self.accumulator = opa;
    }

    // =================V  input/output & RAM instructions V=================

    fn opa_rdm(&mut self) {
        self.accumulator = self.ram_read_nibble();
    }

    // =================V accumulator group instructions in order V=================

    fn opa_clb(&mut self) {
        self.accumulator = 0;
        self.carry = false;
    }

    fn opa_clc(&mut self) {
        self.carry = false;
    }

    fn opa_iac(&mut self) {
        self.accumulator += 1;

        if self.accumulator > 0b1111 {
            self.accumulator = 0;
            self.carry = true;
        } else {
            self.carry = false;
        }
    }

    fn opa_cmc(&mut self) {
        self.carry = !self.carry;
    }

    fn opa_cma(&mut self) {
        self.accumulator = !self.accumulator & 0b1111;
    }

    fn opa_ral(&mut self) {
        self.accumulator <<= 1;
        if self.carry {
            self.accumulator += 1;
        }
        self.carry = match self.accumulator & 0b10000 {
            0b10000 => true,
            _       => false,
        };
        self.accumulator &= 0b1111;
    }

    fn opa_rar(&mut self) {
        let tmp = self.accumulator & 1 == 1;
        self.accumulator >>= 1;
        if self.carry {
            self.accumulator &= 0b1000;
        }
        self.carry = tmp;
    }

    fn opa_tcc(&mut self) {
        self.accumulator = match self.carry {
            false => 0,
            true  => 1,
        };
        self.carry = false;
    }

    fn opa_dac(&mut self) {
        if self.accumulator == 0 {
            self.accumulator = 0b1111;
            self.carry = false;
        } else {
            self.accumulator -= 1;
            self.carry = true;
        }
    }

    fn opa_tcs(&mut self) {
        self.accumulator = match self.carry {
            false => 0b1001,
            true  => 0b1010,
        };
        self.carry = false;
    }

    fn opa_stc(&mut self) {
        self.carry = true;
    }

    fn opa_daa(&mut self) {
        if self.carry && self.accumulator > 9 {
            self.accumulator += 6;
        }

        if self.accumulator > 15 {
            self.accumulator -= 16;
            self.carry = true;
        }
    }

    fn opa_kbp(&mut self) {
        self.accumulator = match self.accumulator {
            0b0000 => 0b0000,
            0b0001 => 0b0001,
            0b0010 => 0b0010,
            0b0100 => 0b0011,
            0b1000 => 0b0100,
            _      => 0b1111,
        };
    }

    fn opa_dcl(&mut self) {
        self.command_control_register = self.accumulator & 0b111;
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "acc: {:x} carry: {} pc: {:03x} pc stack: {:?}\n",
               self.accumulator, self.carry, self.program_counter,
               self.program_counter_stack
               ).unwrap();

        // tidy this up later
        write!(f, "r{:02}: {:x} r{:02}: {:x}   r{:02}: {:x} r{:02}: {:x}   r{:02}: {:x} r{:02}: {:x}   r{:02}: {:x} r{:02}: {:x}\n",
               0, self.index_registers[0], 1, self.index_registers[1],
               2, self.index_registers[2], 3, self.index_registers[3],
               4, self.index_registers[4], 5, self.index_registers[5],
               6, self.index_registers[6], 7, self.index_registers[7]).unwrap();

        write!(f, "r{:02}: {:x} r{:02}: {:x}   r{:02}: {:x} r{:02}: {:x}   r{:02}: {:x} r{:02}: {:x}   r{:02}: {:x} r{:02}: {:x}\n",
               8, self.index_registers[8], 9, self.index_registers[9],
               10, self.index_registers[10], 11, self.index_registers[11],
               12, self.index_registers[12], 13, self.index_registers[13],
               14, self.index_registers[14], 15, self.index_registers[15]).unwrap();

        Ok(()) // wrong, fix properly.
    }
}
