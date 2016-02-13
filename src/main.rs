
mod cpu;
mod rom;
mod ram;
mod hardware;

use std::env;
use std::fs;
use std::io::Read;

fn main() {

    let rom_file_name = env::args().nth(1).unwrap();
    let rom = read_rom(&rom_file_name);

    let hardware = hardware::Hardware::new(rom);
    let mut cpu = cpu::CPU::new(hardware);
    cpu.run();

}

fn read_rom(file_name: &str) -> Vec<u8>{
    let mut file = fs::File::open(file_name).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}
