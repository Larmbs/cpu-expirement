
pub trait RAM {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, value: u8);
}

pub struct RAM64KB {
    values: [u8; u16::MAX as usize],
}
impl RAM64KB {
    /// Create a RAM module
    pub fn new() -> Self {
        Self {
            values: [0u8; u16::MAX as usize]
        }
    }
    /// Loads values from a predefined value array
    pub fn from_arr(values: [u8; u16::MAX as usize]) -> Self {
        Self {values}
    }
}
impl RAM for RAM64KB {
    /// Reads a value from RAM
    fn read(&self, addr: usize) -> u8 {
        assert!(addr < self.values.len(), "Addr {} does not exist, RAM size is {} bytes", addr, self.values.len());
        self.values[addr]
    }
    /// Writes a value to RAM
    fn write(&mut self, addr: usize, value: u8) {
        assert!(addr < self.values.len(), "Addr {} does not exist, RAM size is {} bytes", addr, self.values.len());
        self.values[addr] = value;
    }
}
