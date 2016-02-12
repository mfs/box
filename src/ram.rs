// 4002 RAM
// 320 bits arranged as 4 registers of 20 x 4 bit chars.
// 20 chars are made up of 16 main and 4 status

const NUM_OF_REGISTERS: usize = 4;
const MAIN_MEM_SIZE: usize = 16;
const STATUS_MEM_SIZE: usize = 4;

#[derive(Copy, Clone, Debug)]
struct Register {
    main: [u8; MAIN_MEM_SIZE],     // 16 x 4bit chars main memory
    status: [u8; STATUS_MEM_SIZE], //  4 x 4bit status chars
    output: u8                     //  1 x 4bit output port
}

#[derive(Debug)]
pub struct Ram {
    registers: [Register; NUM_OF_REGISTERS]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            registers: [
                Register {
                    main: [0; MAIN_MEM_SIZE],
                    status: [0; STATUS_MEM_SIZE],
                    output: 0
                };
            NUM_OF_REGISTERS ]
        }
    }

    pub fn read_char(&self, register: u8, character: u8) -> u8 {
        self.registers[register as usize].main[character as usize]
    }

    pub fn read_status(&self, register: u8, status: u8) -> u8 {
        self.registers[register as usize].status[status as usize]
    }
}
