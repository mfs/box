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
            hardware: hardware
        }
    }

    pub fn run(&mut self) {
        loop {
            println!("{:#?}", self);
            self.run_instruction();
            sleep(Duration::from_millis(10));
        }
    }

    fn run_instruction(&mut self) {
        let (opr, opa) = self.rom_read_nibbles(self.program_counter);
        self.program_counter += 1;

        match opr {
            0x0 => { // NOP
                // what if opa != 0? Still a NOP?
            },
            0x2 => { // FIM
                let r0 = opa;
                let (d2, d1) = self.rom_read_nibbles(self.program_counter);
                // could make a write_register_pair() function.
                self.index_registers[r0 as usize] = d2;
                self.index_registers[(r0 + 1) as usize] = d1;
                self.program_counter += 1;
            },
            0x4 => { // JUN
                let (a2, a1) = self.rom_read_nibbles(self.program_counter);
                self.program_counter = ((opa as u16) << 8) +
                                       ((a2 as u16) << 4) +
                                       a1 as u16;
            },
            0x8 => { // ADD
                let mut sum: u8 = self.index_registers[opa as usize] + self.accumulator;
                if self.carry { sum += 1; }
                self.accumulator = sum & 0b1111;
                if sum > 15 { self.carry = true; }
            },
            0xa => { // LD
                self.accumulator = self.index_registers[opa as usize];
            },
            0xb => { // XCH
                let tmp: u8 = self.accumulator;
                self.accumulator = self.index_registers[opa as usize];
                self.index_registers[opa as usize] = tmp;
            },
            _   => panic!("Unrecognized instruction: {:0x}{:0x}", opr, opa),
        }
    }

    fn rom_read_nibbles(&self, address: u16) -> (u8, u8) {
        let byte = self.hardware.rom_read_byte(address);

        ((byte >> 4) & 0b1111, byte & 0b1111)
    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "acc: {} carry: {} pc: {} pc1: {} pc2: {} pc3: {}\nir: {:?}",
               self.accumulator, self.carry, self.program_counter,
               self.program_counter_1, self.program_counter_2,
               self.program_counter_3, self.index_registers)
    }
}
