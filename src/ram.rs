


pub struct RAM {
    values: [u8; u8::MAX as usize],
}
impl RAM {
    /// Create a RAM module
    pub fn new() -> Self {
        Self {
            values: [0u8; u8::MAX as usize]
        }
    }
    /// Loads values from a predefined value array
    pub fn from_arr(values: [u8; u8::MAX as usize]) -> Self {
        Self {values}
    }
    /// Reads a value from RAM
    pub fn read(&self, addr: usize) -> u8 {
        assert!(addr < self.values.len(), "Addr {} does not exist, RAM size is {} bytes", addr, self.values.len());
        self.values[addr]
    }
    /// Writes a value to RAM
    pub fn write(&mut self, addr: usize, value: u8) {
        assert!(addr < self.values.len(), "Addr {} does not exist, RAM size is {} bytes", addr, self.values.len());
        self.values[addr] = value;
    }
}
