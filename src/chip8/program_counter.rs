use crate::chip8::core::Chip8;
use crate::instruction::Instruction;

pub trait ProgramCounter {
    fn fetch_instruction(&mut self) -> Option<Instruction>;
    fn read_instruction_bytes(&self) -> Option<[u8; 2]>;
    fn increment_program_counter(&mut self);
    fn end_of_source(&self) -> bool;
}

impl ProgramCounter for Chip8 {
    fn fetch_instruction(&mut self) -> Option<Instruction> {
        let bytes = self.read_instruction_bytes()?;
        let address = self.program_counter;
        self.increment_program_counter();
        Some(Instruction::new(bytes, address))
    }

    fn read_instruction_bytes(&self) -> Option<[u8; 2]> {
        let pc = self.program_counter as usize;

        if self.end_of_source() { return None }

        Some([self.memory[pc], self.memory[pc + 1]])
    }

    fn increment_program_counter(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(2);
    }

    fn end_of_source(&self) -> bool {
        (self.program_counter as usize + 1) >= 0x200 + self.source.bytes.len()
    }
}