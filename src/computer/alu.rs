//! Module that defines different implementations of an ALU
use crate::{
    never_happens,
    is_bit_len,
};

pub struct ALU;
impl ALU {
    /// Executes an ALU operation
    /// 
    /// Requires an 3bit number for mode
    /// This specifies what operation to conduct
    #[allow(non_snake_case)]
    pub fn exec(A: u8, B: u8, mode: u8) -> u8 {
        assert!(is_bit_len(2, mode as usize), "Instruction must be 2bits in length");

        match mode {
            0b000 => A | B,    // Bitwise OR between A and B
            0b001 => !A,       // Bitwise NOT on A
            0b010 => A ^ B,    // Bitwise XOR between A and B
            0b011 => A & B,    // Bitwise AND between A and B
            0b100 => A << B,   // Right Shift on A by B
            0b101 => A >> B,   // Left Shift on A by B
            0b110 => A + B,    // Bitwise addition between A and B
            0b111 => A - B,    // Bitwise subtraction on A by B
            _ => never_happens(),
        }
    }
}
