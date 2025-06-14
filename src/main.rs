mod chip8;
mod address;
// mod screen;

use chip8::Chip8;
use address::Address;
// use screen::Screen;


fn main() {
    let chip8 = Chip8::new();
    // Fetch
    // Decode
    // Evaluate
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn testing_works() {
        assert_eq!(5, 5)
    }
}