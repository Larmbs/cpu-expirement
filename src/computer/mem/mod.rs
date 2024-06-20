//! Module which defines different memory storage and writing methods

mod rom;
pub use rom::{ROM, ROM1KB, ROM64KB};

mod ram;
pub use ram::{RAM, RAM64KB};

/// Returns a formatted error message when address specified is not supported
fn address_out_of_range(len: usize) -> String {
    format!("Exceeded maximum supported address, max is {}", len - 1)
}
