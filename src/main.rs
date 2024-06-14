mod ram;
mod alu;
mod cpu;

fn main() {
    let alu = alu::ALU;
    let mut ram = ram::RAM::new();
    println!("Hello, world!");
}
