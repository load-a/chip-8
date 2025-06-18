mod core;
pub mod memory_management;
pub mod flag_register;
pub mod program_counter;
pub mod decoder;

#[cfg(test)]
mod test;

pub use core::Chip8;
