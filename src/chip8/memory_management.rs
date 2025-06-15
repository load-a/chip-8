use crate::chip8::core::Chip8;
use crate::address::Address;
use crate::instruction::Instruction;

pub trait MemoryManagement {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
    fn read_address(&self, destination: u16) -> Address;
    fn write_address(&mut self, destination: u16, address: &Address);
    fn read_instruction_bytes(&self) -> [u8; 2];
    fn increment_program_counter(&mut self);
    fn fetch_instruction(&mut self) -> Instruction;
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

    fn read_instruction_bytes(&self) -> [u8; 2] {
        let first_byte = self.program_counter as usize;
        let second_byte = self.program_counter.wrapping_add(1) as usize;

        [self.memory[first_byte], self.memory[second_byte]]
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(2);
    }

    fn fetch_instruction(&mut self) -> Instruction {
        let bytes = self.read_instruction_bytes();
        let address = self.program_counter;

        self.increment_program_counter();
        Instruction::new(bytes, address)
    }
}