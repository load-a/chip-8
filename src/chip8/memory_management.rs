use crate::chip8::core::Chip8;
use crate::address::Address;

pub trait MemoryManagement {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
    fn read_address(&self, destination: u16) -> Address;
    fn write_address(&mut self, destination: u16, address: &Address);
}

impl MemoryManagement for Chip8 {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    fn read_address(&self, destination: u16) -> Address {
        let high_byte = destination as usize;
        let low_byte = (destination + 1) as usize;

        Address::from_bytes([self.memory[high_byte], self.memory[low_byte]])
    }

    fn write_address(&mut self, destination: u16, address: &Address) {
        let high_byte = destination as usize;
        let low_byte = (destination + 1) as usize;

        self.memory[high_byte] = address.high_byte;
        self.memory[low_byte] = address.low_byte;
    }
}