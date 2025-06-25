mod chip8;
mod address;
mod screen;
mod instruction;
mod source;

use chip8::{
    Chip8, 
    decoder::Decoder,
    flag_register::FlagRegister,
    program_counter::ProgramCounter,
};
use source::Source;

// use address::Address;
use minifb::Key;
use instruction::Instruction;


fn main() {
    let source = Source::new();
    let mut chip8 = Chip8::new(Some(source));

    chip8.screen.dev_pattern();

    while chip8.screen.is_running() {
        if chip8.end_of_source() { 
            break 
        } else {
            // Fetch
            let instruction = chip8.fetch_instruction();
            // Decode
            chip8.decode(instruction);
            // Evaluate
            chip8.screen.update();
        }
    }
}
