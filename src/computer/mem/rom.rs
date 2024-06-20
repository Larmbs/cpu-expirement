//! Module which defines what ROM is and some implementations of various sized ROM units

use super::address_out_of_range;

/// Trait that defines ROM behavior
pub trait ROM {
    fn new(mem: &[u8]) -> Self;
    fn read(&self, addr: usize) -> u8;
}

/// Definition for 64KB ROM
pub struct ROM64KB {
    values: [u8; u16::MAX as usize],
}
impl ROM for ROM64KB {
    fn new(mem: &[u8]) -> Self {
        Self { values: mem[0..u16::MAX as usize].try_into().unwrap() }
    }
    fn read(&self, addr: usize) -> u8 {
        assert!(
            addr < self.values.len(),
            "{}",
            address_out_of_range(self.values.len())
        );
        self.values[addr]
    }
}

/// Definition for a 1KB ROM
pub struct ROM1KB {
    values: [u8; u8::MAX as usize],
}
impl ROM for ROM1KB {
    fn new(mem: &[u8]) -> Self {
        Self { values: mem[0..u8::MAX as usize].try_into().unwrap() }
    }
    fn read(&self, addr: usize) -> u8 {
        assert!(
            addr < self.values.len(),
            "{}",
            address_out_of_range(self.values.len())
        );
        self.values[addr]
    }
}
