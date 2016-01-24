
mod cpu;
mod hardware;

fn main() {
    let hardware = hardware::Hardware::new();
    let cpu = cpu::CPU::new(hardware);

    println!("{}", cpu);
}
