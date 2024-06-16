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
    /// 
    /// Requires an 3bit number for mode
    /// This specifies what operation to conduct
    #[allow(non_snake_case)]
    pub fn exec(A: u8, B: u8, mode: u8) -> u8 {
        assert!(mode <= 3, "Instruction must be 2bits in length");

        match mode {
            0b000 => A | B,    // Bitwise OR between A and B
            0b001 => !A,       // Bitwise NOT on A
            0b010 => A ^ B,    // Bitwise XOR between A and B
            0b011 => A & B,    // Bitwise AND between A and B
            0b100 => A << B,   // Right Shift on A by B
            0b101 => A >> B,   // Left Shift on A by B
            0b110 => A + B,    // Bitwise addition between A and B
            0b111 => A - B,    // Bitwise subtraction on A by B
            _ => panic!("This will never happen")
        }
    }
}
