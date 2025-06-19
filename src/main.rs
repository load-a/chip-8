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
use minifb::Key;
use instruction::Instruction;


fn main() {
    let mut chip8 = Chip8::new();

    chip8.memory[0] = 0b11111000;
    chip8.memory[1] = 0b10000000;
    chip8.memory[2] = 0b11000000;
    chip8.memory[3] = 0b10000000;
    chip8.memory[4] = 0b11111000;

    chip8.register[0] = 7;

    chip8.screen.blackout();
    chip8.decode(Instruction::new([0xD0, 0x15], 0));

    while chip8.screen.is_running() {
        chip8.screen.update();
    }

    // Fetch
    // Decode
    // Evaluate
}
