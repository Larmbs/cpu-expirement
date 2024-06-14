use crate::alu::ALU;
use crate::ram::RAM;

/// CPU Instruction
/// Size: 8bit
/// 
/// 0x01010111
/// 
/// First 2bits are reserved for instruction type.
/// 0x00 ALU operation
/// 0x01 Store operation
/// 0x10 Load operation
/// 0x11 ???
/// 
pub struct CPU {
    // Connections to external modules
    alu: ALU,
    ram: RAM,

    // Local registers
    regs: [u8; 4],        // CPU intermediate store registers
    program_counter: u8,  // Register storing program pointer
    aux: u8,              // ALU output register
}

impl CPU {
    /// Creates a CPU containing default values
    pub fn new() -> Self {
        Self {
            alu: ALU,
            ram: RAM::new(),
            regs: [0u8; 4],
            program_counter: 0u8,
            aux: 0u8,
        }
    }
    /// Creates CPU from predefined modules
    pub fn from_custom(alu: ALU, ram: RAM) -> Self {
        Self {
            alu,
            ram,
            regs: [0u8; 4],
            program_counter: 0u8,
            aux: 0u8,
        }
    }
    /// Executes a CPU cycle
    pub fn tick(&mut self) {
        // Reading instruction from RAM
        let instruction = self.ram.read(self.program_counter as usize);

        let op = (0b11000000 & instruction) >> 6; // First 2 bits
        let args = 63 & instruction;     // Last 6 bits

        match op {
            0b00 => todo!(),
            0b01 => todo!(),
            0b10 => todo!(),
            0b11 => todo!(),
            _ => panic!("Will never happen")
        };
    }
}
