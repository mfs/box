
// This will contain all the random hardware required. ROM, RAM, etc. Will
// probably emulate this at a high level. For example I won't emulate
// individual ROM/RAM chips. I'll just emulate the entire memory space and
// bank switching will just be an offset or something.

// The 4004 can control 16 4001 ROMs. Each ROM contains 256 x 8bit words.
// 16 * 256 x 8bit words = 4096 x 8bit words.
use ram::Ram;
use rom::Rom;

const ROM_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Hardware {
    rom: Rom,
    ram: Ram
}

impl Hardware {
    pub fn new(rom: Vec<u8>) -> Hardware {
        assert!(rom.len() <= ROM_SIZE);
        Hardware {
            rom: Rom::new(rom),
            ram: Ram::new()
        }
    }

    pub fn rom_read_byte(&self, address: u16) -> u8 {
        self.rom.read_word(address as u8)
    }

    // only one chip at the moment
    pub fn ram_read_char(&self, _chip: u8, register: u8, character: u8) -> u8 {
        self.ram.read_char(register, character)
    }

    pub fn ram_write_char(&mut self, _chip: u8, register: u8, character: u8, value: u8) {
        self.ram.write_char(register, character, value)
    }

    pub fn ram_read_status(&self, _chip: u8, register: u8, status: u8) -> u8 {
        self.ram.read_status(register, status)
    }

    pub fn ram_write_status(&mut self, _chip: u8, register: u8, status: u8, value: u8) {
        self.ram.write_status(register, status, value)
    }

    pub fn ram_write_output(&mut self, _chip: u8, value: u8) {
        self.ram.write_output(value);
    }
}
