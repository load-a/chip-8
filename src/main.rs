mod chip8;
mod address;
mod screen;
mod instruction;
mod source;

use chip8::{
    Chip8, 
    decoder::Decoder,
    flag_register::FlagRegister,
};
use source::Source;

// use address::Address;
use minifb::Key;
use instruction::Instruction;


fn main() {
    let mut chip8 = Chip8::new();
    let source = Source::new();

    while chip8.screen.is_running() {
        // Fetch
        // Decode
        // Evaluate
        chip8.screen.update();
    }
}
