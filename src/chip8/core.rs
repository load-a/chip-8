use crate::chip8::{
    flag_register,
    memory_management,
    program_counter,
    decoder,
};
use crate::screen::Screen;
use crate::source::Source;

#[derive(Debug)]
pub struct Chip8 {
    pub index_register: u16,
    pub program_counter: u16,
    pub register: [u8; 16],
    pub memory: [u8; 4096],
    pub screen: Screen,
    pub timer: u8,
    pub sound: u8,
    pub stack_pointer: u8,
    pub keys: [bool; 16],
    pub source: Source,
    stack: [u16; 16]
}

impl Chip8 {
    pub fn new(source: Option<Source>) -> Self {
        let mut instance = Chip8 {
            index_register: 0,
            program_counter: 0x200,
            register: [0; 16],
            memory: [0; 4096],
            screen: Screen::new(),
            timer: 0,
            sound: 0,
            stack_pointer: 0x0,
            keys: [false; 16],
            source: source.unwrap_or(Source::default()),
            stack: [0; 16]
        };

        instance.copy_source();

        instance
    }

    fn copy_source(&mut self) {
        let start = 0x200;
        let end = start + self.source.bytes.len();

        self.memory[start..end].copy_from_slice(&self.source.bytes);
    }

    pub fn push_pc(&mut self) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
    }

    pub fn pop_pc(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }
}
