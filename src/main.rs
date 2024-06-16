mod computer;
use computer::*;

fn main() {
    let alu = ALU;
    let mut ram = RAM64KB::new();

    let mut cpu = CPU::from_custom(alu, ram);

    
    println!("Hello, world!");
}
