use std::time::{Instant, Duration};

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
    let mut last_timer_update = Instant::now();
    let mut last_cycle_time = Instant::now();

    let cycle_duration = Duration::from_micros(1_000_000 / 700);    // 700 Hz
    let timer_duration = Duration::from_millis(1_000 / 60);         // 60 Hz

    chip8.screen.blackout();

    while chip8.screen.is_running() {
        let now = Instant::now();

        if now - last_cycle_time >= cycle_duration {
            if chip8.end_of_source() { break }

            // Fetch
            let instruction = chip8.fetch_instruction();
            // Decode & Evaluate
            chip8.decode(instruction);
            last_cycle_time = now;
        }

        if now - last_timer_update >= timer_duration {
            chip8.decrement_timers();
            last_timer_update = now;
        }

        chip8.screen.update();
        if chip8.sound > 0 { chip8.screen.play_sound() } else  { chip8.screen.stop_sound() };
    }
}
