use crate::chip8::core::Chip8;
use crate::instruction::Instruction;

pub trait ProgramCounter {
    fn read_instruction_bytes(&self) -> [u8; 2];
    fn increment_program_counter(&mut self);
    fn fetch_instruction(&mut self) -> Instruction;
}

impl ProgramCounter for Chip8 {
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