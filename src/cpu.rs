use std::fmt;
use std::thread::sleep;
use std::time::Duration;
use hardware::Hardware;

// The 4004 is a 4 bit data / 12 bit address CPU therefore it doesn't really
// fit into the standard integer types. Comments below show actual size of the
// registers. Not sure how to best handle this.

#[derive(Debug)]
pub struct CPU {               // actual register size
    accumulator: u8,           // u4
    carry: bool,               // u1

    program_counter: u16,      // u12

    program_counter_1: u16,    // u12
    program_counter_2: u16,    // u12
    program_counter_3: u16,    // u12

    index_registers: [u8; 16], // u4

    // internal register used for ram bank switching
    command_control_register: u8,

    hardware: Hardware
}

impl CPU {
    pub fn new(hardware: Hardware) -> CPU {
        CPU {
            accumulator: 0,
            carry: false,
            program_counter: 0,
            program_counter_1: 0,
            program_counter_2: 0,
            program_counter_3: 0,
            index_registers: [0; 16],
            command_control_register: 0,
            hardware: hardware
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("{}", self);
            self.run_instruction();
            sleep(Duration::from_millis(10));
        }
    }

    fn run_instruction(&mut self) {
        let (opr, opa) = self.rom_read_nibbles(self.program_counter);
        self.program_counter += 1;

        match opr {
            0x0 => { }, // NOP What if opa != 0? Still a NOP?
            0x2 => self.opr_fin(opa),
            0x4 => self.opr_jun(opa),
            0x8 => self.opr_add(opa),
            0xa => self.opr_ld(opa),
            0xb => self.opr_xch(opa),
            0xd => self.opr_ldm(opa),
            0xf => { // Accumulator Group Instructions
                match opa {
                    0x0 => { self.accumulator = 0; self.carry = false; }, // CLB
                    0x1 => { self.carry = false;  }, // CLC
                    0x3 => { self.carry = !self.carry;  }, // CMC
                    0x5 => self.opa_ral(),
                    0x6 => self.opa_rar(),
                    0x7 => self.opa_tcc(),
                    0x9 => self.opa_tcs(),
                    0xa => { self.carry = true;  }, // STC
                    0xb => self.opa_daa(),
                    0xc => self.opa_kbp(),
                    0xd => self.opa_dcl(),
                    _   => panic!("Unrecognized instruction: {:0x}{:0x}", opr, opa),
                }
            },
            _   => panic!("Unrecognized instruction: {:0x}{:0x}", opr, opa),
        }
    }

    fn rom_read_nibbles(&self, address: u16) -> (u8, u8) {
        let byte = self.hardware.rom_read_byte(address);

        ((byte >> 4) & 0b1111, byte & 0b1111)
    }

    // =========================V operands in order V=========================

    fn opr_fin(&mut self, opa: u8) {
        let (d2, d1) = self.rom_read_nibbles(self.program_counter);
        // could make a write_register_pair() function.
        self.index_registers[opa as usize] = d2;
        self.index_registers[(opa + 1) as usize] = d1;
        self.program_counter += 1;
    }

    fn opr_jun(&mut self, opa: u8) {
        let (a2, a1) = self.rom_read_nibbles(self.program_counter);
        self.program_counter = ((opa as u16) << 8)
                             + ((a2 as u16) << 4)
                             + a1 as u16;
    }

    fn opr_add(&mut self, opa: u8) {
        let mut sum: u8 = self.index_registers[opa as usize] + self.accumulator;
        if self.carry { sum += 1; }
        self.accumulator = sum & 0b1111;
        if sum > 15 { self.carry = true; }
    }

    fn opr_ld(&mut self, opa: u8) {
        self.accumulator = self.index_registers[opa as usize];
    }

    fn opr_xch(&mut self, opa: u8) {
        let tmp: u8 = self.accumulator;
        self.accumulator = self.index_registers[opa as usize];
        self.index_registers[opa as usize] = tmp;
    }

    fn opr_ldm(&mut self, opa: u8) {
        self.accumulator = opa;
    }

    // =================V accumulator group instructions in order V=================


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

    fn opa_tcs(&mut self) {
        self.accumulator = match self.carry {
            false => 0b1001,
            true  => 0b1010,
        };
        self.carry = false;
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
        write!(f, "acc: {:x} carry: {} pc: {:03x} pc1: {:03x} pc2: {:03x} pc3: {:03x}\n",
               self.accumulator, self.carry, self.program_counter,
               self.program_counter_1, self.program_counter_2,
               self.program_counter_3).unwrap();

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
