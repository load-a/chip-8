use crate::chip8::core::Chip8;

pub trait FlagRegister {
    fn flag_register(&self) -> u8;
    fn set_flag_register(&mut self);
    fn reset_flag_register(&mut self);
}

impl FlagRegister for Chip8 {
    fn flag_register(&self) -> u8 {
        self.register[15]
    }

    fn set_flag_register(&mut self) {
        self.register[15] = 1;
    }

    fn reset_flag_register(&mut self) {
        self.register[15] = 0;
    }
}