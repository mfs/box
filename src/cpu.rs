use std::fmt;
use hardware::Hardware;

// The 4004 is a 4 bit data / 12 bit address CPU therefore it doesn't really
// fit into the standard integer types. Comments below show actual size of the
// registers. Not sure how to best handle this.

pub struct CPU {                   // actual register size
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
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "acc: {} carry: {} pc: {} pc1: {} pc2: {} pc3: {}\nir: {:?}",
               self.accumulator, self.carry, self.program_counter,
               self.program_counter_1, self.program_counter_2,
               self.program_counter_3, self.index_registers)
    }
}
