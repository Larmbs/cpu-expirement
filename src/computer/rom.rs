
pub trait ROM {
    fn read(&self, addr: usize) -> u8;
}
pub struct ROM64KB {
    values: [u8; u16::MAX as usize]
}
impl ROM64KB {
}

pub struct ROM1KB {
    values: [u8; u8::MAX as usize]
}
impl ROM1KB {
}