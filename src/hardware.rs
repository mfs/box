
// This will contain all the random hardware required. ROM, RAM, etc. Will
// probably emulate this at a high level. For example I won't emulate
// individual ROM/RAM chips. I'll just emulate the entire memory space and
// bank switching will just be an offset or something.

// The 4004 can control 16 4001 ROMs. Each ROM contains 256 x 8bit words.
// 16 * 256 x 8bit words = 4096 x 8bit words.
use ram::Ram;

const ROM_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Hardware {
    rom: Vec<u8>,
    ram: Ram
}

impl Hardware {
    pub fn new(rom: Vec<u8>) -> Hardware {
        assert!(rom.len() <= ROM_SIZE);
        Hardware {
            rom: rom,
            ram: Ram::new()
        }
    }

    pub fn rom_read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    // only one chip at the moment
    pub fn ram_read_char(&self, _chip: u8, register: u8, character: u8) -> u8 {
        self.ram.read_char(register, character)
    }
}
