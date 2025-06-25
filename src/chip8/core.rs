use crate::chip8::flag_register;
use crate::chip8::memory_management::MemoryManagement;
use crate::chip8::program_counter;
use crate::chip8::decoder;

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
    pub stack_pointer: u16,
    pub keys: [bool; 16],
    pub source: Source,
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
            stack_pointer: 0x1ff,
            keys: [false; 16],
            source: source.unwrap_or(Source::default()),
        };

        instance.copy_source();

        instance
    }

    fn copy_source(&mut self) {
        let start = 0x200;
        let end = start + self.source.bytes.len();

        self.memory[start..end].copy_from_slice(&self.source.bytes);
    }
}
