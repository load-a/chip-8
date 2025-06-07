mod chip8;
mod address;

use chip8::Chip8;
use address::Address;

fn main() {
    let _chip8 = Chip8::new();
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