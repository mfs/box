
// This will contain all the random hardware required. ROM, RAM, etc. Will
// probably emulate this at a high level. For example I won't emulate
// individual ROM/RAM chips. I'll just emulate the entire memory space and
// bank switching will just be an offset or something.

pub struct Hardware {
    // The 4004 can control 16 4001 ROMs. Each ROM contains 256 x 8bit words.
    // 16 * 256 x 8bit words = 4096 x 8bit words.
    rom: [u8; 4096]
}

impl Hardware {
    pub fn new() -> Hardware {
        Hardware {
            rom: [0; 4096]
        }
    }
}
