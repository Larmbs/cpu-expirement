//!
//! 
//! 

/// 8bit ALU component
/// 
/// Instruction set is based on this article
/// https://www.cs.uic.edu/~i266/fall12_hw10/5571.pdf
/// 

pub struct ALU;
impl ALU {
    /// Executes an ALU operation
    #[allow(non_snake_case)]
    pub fn exec(A: u8, B: u8, mode: u8, cin: u8) -> u8 {
        assert!(mode <= 7, "Instruction must be 3bits in length");
        assert!(cin <= 1, "Carry in must be a single bit");

        match mode {
            0 => A | B,    // Bitwise OR
            1 => !A,       // Bitwise NOT on A
            2 => A + !B + cin,
            3 => A + B + cin,
            4 => A ^ B,    // Bitwise XOR
            5 => A & B,    // Bitwise AND
            6 => A - 1 + cin,
            7 => A + cin,
            _ => panic!("This will never happen")
        }
    }
}
