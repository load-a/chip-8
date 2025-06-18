mod chip8;
mod address;
mod screen;
mod instruction;

use chip8::{
    Chip8, 
    decoder::Decoder,
    flag_register::FlagRegister,
};

// use address::Address;
// use screen::Screen;
use instruction::Instruction;


fn main() {
    let mut chip8 = Chip8::new();

    chip8.screen.default_pattern();

    while chip8.screen.is_running() {
        chip8.decode(Instruction::new([0x00, 0xe0], 0));

        chip8.screen.update();
    }

    // Fetch
    // Decode
    // Evaluate
}
