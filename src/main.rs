mod chip8;
mod address;
mod screen;

use chip8::Chip8;
use address::Address;
use screen::Screen;


fn main() {
    let mut chip8 = Chip8::new();

    while chip8.screen.is_running() {
        chip8.screen.default_pattern();

        chip8.screen.update();
    }

    // chip8.screen.
    // Fetch
    // Decode
    // Evaluate
}

// #[cfg(test)]
// mod tests {
//     // use super::*;

//     #[test]
//     fn testing_works() {
//         assert_eq!(5, 5)
//     }
// }