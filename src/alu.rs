//!
//! 
//! 

/// 8bit ALU component
/// 
/// Instruction set is based on this article
/// https://www.cs.uic.edu/~i266/fall12_hw10/5571.pdf
/// 
/// A = 8bit value
/// B = 8bit value
/// 
/// cin = Carry In
/// 
/// N | Mode | Operation
/// --+------+----------
/// 0 | 0b000| A | B
/// 1 | 0b001| !A
/// 2 | 0b010| A + !B+ cin
/// 3 | 0b011| A + B + cin
/// 4 | 0b100| A ^ B
/// 5 | 0b101| A & B
/// 6 | 0b110| A - 1 + cin
/// 7 | 0b111| A + cin
/// 

pub struct ALU;
impl ALU {
    /// Executes an ALU operation
    #[allow(non_snake_case)]
    pub fn exec(A: u8, B: u8, mode: u8, cin: u8) -> u8 {
        assert!(mode <= 7, "Instruction must be 3bits in length");
        assert!(cin <= 1, "Carry in must be a single bit");

        match mode {
            0b000 => A | B,    // Bitwise OR
            0b001 => !A,       // Bitwise NOT on A
            0b010 => A + !B + cin,
            0b011 => A + B + cin,
            0b100 => A ^ B,    // Bitwise XOR
            0b101 => A & B,    // Bitwise AND
            0b110 => A - 1 + cin,
            0b111 => A + cin,
            _ => panic!("This will never happen")
        }
    }
}
