//! Module which defines what RAM is and some implementations of various sized RAM units

use super::address_out_of_range;

/// Trait that defines RAM behavior
pub trait RAM {
    /// Creates RAM full of zeroes
    fn new() -> Self;
    /// Creates RAM from a predefined array, panics if array provided is wrong size
    fn from_values(mem: &[u8]) -> Self;
    /// Reads a value from RAM
    fn read(&self, addr: usize) -> u8;
    /// Writes a value to RAM
    fn write(&mut self, addr: usize, value: u8);
}

/// Definition for 64KB RAM
pub struct RAM64KB {
    mem: [u8; u16::MAX as usize],
}
impl RAM for RAM64KB {
    fn new() -> Self {
        Self {mem: [0u8; u16::MAX as usize],}
    }
    fn from_values(mem: &[u8]) -> Self {
        Self {
            mem: mem[0..u16::MAX as usize].try_into().unwrap(),
        }
    }
    fn read(&self, addr: usize) -> u8 {
        assert!(
            addr < self.mem.len(),
            "{}",
            address_out_of_range(self.mem.len())
        );
        self.mem[addr]
    }
    fn write(&mut self, addr: usize, value: u8) {
        assert!(
            addr < self.mem.len(),
            "{}",
            address_out_of_range(self.mem.len())
        );
        self.mem[addr] = value;
    }
}

/// Definition for 1KB RAM
pub struct RAM1KB {
    mem: [u8; u16::MAX as usize],
}
impl RAM for RAM1KB {
    fn new() -> Self {
        Self {
            mem: [0u8; u16::MAX as usize],
        }
    }
    fn from_values(mem: &[u8]) -> Self {
        Self {
            mem: mem[0..u8::MAX as usize].try_into().unwrap(),
        }
    }
    fn read(&self, addr: usize) -> u8 {
        assert!(
            addr < self.mem.len(),
            "{}",
            address_out_of_range(self.mem.len())
        );
        self.mem[addr]
    }
    fn write(&mut self, addr: usize, value: u8) {
        assert!(
            addr < self.mem.len(),
            "{}",
            address_out_of_range(self.mem.len())
        );
        self.mem[addr] = value;
    }
}
