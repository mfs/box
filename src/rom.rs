// 4001 ROM
// 256 x 8bit words
// 4 x IO pins

// The IO is incomplete. In reality the IO pins can be configured as input or
// output. In both cases they can be direct or inverted. Pull up/down resistors
// can be used in both cases. You can set an IO pin that is an output to return
// a particular level when read. This is all set at manufacture. See the MCS
// manual for more the full story. I don't plan on using the IO pins for now
// so am just modeling them as 4 bits. I'll work out the details later.

use std;

const ROM_SIZE: usize = 256;

pub struct Rom {
    words: [u8; ROM_SIZE],
    in_out: u8
}

impl std::fmt::Debug for Rom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "-- rom --")
    }
}

impl Rom {
    pub fn new(rom: Vec<u8>) -> Rom {
        assert!(rom.len() <= ROM_SIZE);
        let mut r = Rom {
            words: [0; ROM_SIZE],
            in_out: 0
        };

        for x in 0..rom.len() {
            r.words[x] = rom[x];
        }

        r
    }

    pub fn read_word(&self, address: u8) -> u8 {
        self.words[address as usize]
    }
}
