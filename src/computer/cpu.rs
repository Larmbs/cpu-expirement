use super::alu::ALU;
use super::{RAM64KB, RAM};

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
    ram: RAM64KB,

    // Local registers
    regs: [u8; 4],        // CPU intermediate store registers
    program_counter: u8,  // Register storing program pointer
    aux: u8,              // ALU output register

    addr_register: u8,   // Address register
}

impl CPU {
    /// Creates a CPU containing default values
    pub fn new() -> Self {
        Self {
            alu: ALU,
            ram: RAM64KB::new(),
            regs: [0u8; 4],
            program_counter: 0u8,
            aux: 0u8,

            addr_register: 0,
        }
    }
    /// Creates CPU from predefined modules
    pub fn from_custom(alu: ALU, ram: RAM64KB) -> Self {
        Self {
            alu,
            ram,
            regs: [0u8; 4],
            program_counter: 0u8,
            aux: 0u8,
            addr_register: 0,
        }
    }
    /// Executes a CPU cycle
    pub fn tick(&mut self) {
        // Reading instruction from RAM
        let instruction = self.ram.read(self.program_counter as usize);

        let op = (0b11000000 & instruction) >> 6; // First 2 bits
        let args = 0b00111111 & instruction;              // Last  6 bits

        match op {
            // 0b00 when doing binary operations 0b01 when doing arithmetic
            0b00 | 0b01 => {
                // Will always be 2bit indices
                let a = self.regs[(args & 0b00000011) as usize];
                let b = self.regs[((args & 0b00001100) >> 2) as usize];

                // Cal to get 3bit number to specify mode
                let mode = ((args & 0b00110000) >> 4) + op << 2;

                // Sets output register
                self.aux = ALU::exec(a, b, mode);
            },
            // STORE and LOAD
            0b10 => {
                
                let value = self.regs[0];
                self.ram.write(self.addr_register as usize, value);
            },
            0b11 => todo!(),
            _ => panic!("Will never happen")
        };
    }
    
}
