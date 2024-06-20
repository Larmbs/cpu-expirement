#![allow(unused, unused_imports)]

mod cpu;
pub use cpu::CPU;

mod alu;
pub use alu::ALU;

mod mem;
pub use mem::*;

/// Exit function that will prob never happen
pub fn never_happens() -> ! {
    panic!("This will never ever happen, congratulations if it did!")
}

/// Checks if value found can be contained within target bit len
pub fn is_bit_len(target: usize, found: usize) -> bool {
    (1 << target) > found
}
