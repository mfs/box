// The 4004 is a 4 bit data / 12 bit address CPU therefore it doesn't really
// fit into the standard integer types. Comments below show actual size of the
// registers. Not sure how to best handle this.

#[derive(Default, Debug)]
struct CPU {                   // actual register size
    accumulator: u8,           // u4
    carry: bool,               // u1

    program_counter: u16,      // u12

    program_counter_1: u16,    // u12
    program_counter_2: u16,    // u12
    program_counter_3: u16,    // u12

    index_registers: [u8; 16], // u4
}


fn main() {
    let cpu: CPU = Default::default();

    println!("{:?}", cpu);
}
