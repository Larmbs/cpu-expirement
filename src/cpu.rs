use crate::alu::ALU;
use crate::ram::RAM;

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
    /// Creates a cpu containing default values
    pub fn new() -> Self {
        Self {
            alu: ALU,
            ram: RAM::new(),
            regs: [0u8; 4],
            program_counter: 0u8,
            aux: 0u8,
        }
    }

    pub fn tick(&mut self) {


        self.program_counter += 1;
    }
}
