mod ram;
mod alu;
mod cpu;

fn main() {
    let alu = alu::ALU;
    let mut ram = ram::RAM::new();

    let mut cpu = cpu::CPU::from_custom(alu, ram);

    
    println!("Hello, world!");
}
