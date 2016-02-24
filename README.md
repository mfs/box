[![Build Status](https://travis-ci.org/mfs/box.svg?branch=master)](https://travis-ci.org/mfs/box)

I may have caught the 'write an emulator' bug. This is just a place holder for
now. The plan is to write a simple emulator of an Intel 4004. I'll add some
memory mapped io for a display. Probably character based at this point.

Testing the emulator will require ROM images which seem fairly scarce. I'll make
some ROMs using the assembler located [here][szyc].

I wasn't intending to emulate the individual RAM chips though I may end up doing
this due to the close relationship of some of the instructions to the hardware.

### Data Types

The 4004 is a 4 bit CPU. Characters are 4 bit, words are 8 bit, addresses are 12 bit.

| Type    | Bits | Rust Type |
|---------|-----:|----------:|
| char    | 4    | u8        |
| word    | 8    | u8        |
| address | 12   | u16       |

[szyc]: http://e4004.szyc.org/index_en.html
