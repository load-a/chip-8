use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::chip8::core::Chip8;

pub trait Inputs {
    fn any_key_pressed(&self) -> bool;
    fn check_key(&self, name: String) -> bool;
    fn string_to_keycode(string: String) -> Option<Keycode>;
}

impl Inputs for Chip8 {
    fn any_key_pressed(&self) -> bool {
        let device = DeviceState::new();
        let keyscan: Vec<Keycode> = device.get_keys();

        let valid_keys = [
            "1", "2", "3", "C",
            "4", "5", "6", "D",
            "7", "8", "9", "E",
            "A", "0", "B", "F",
        ];

        for key in valid_keys {
            if Self::check_key(&self, String::from(key)) {
                return true;
            }
        }

        false
    }

    fn check_key(&self, name: String) -> bool {
        let device = DeviceState::new();
        let keyscan: Vec<Keycode> = device.get_keys();

        if let Some(code) = Self::string_to_keycode(name) {
            keyscan.contains(&code)
        } else {
            false
        }
    }

    fn string_to_keycode(key: String) -> Option<Keycode> {
        use Keycode::*;

        Some(match key.to_uppercase().as_str() {
            "1" => Key1,
            "2" => Key2,
            "3" => Key3,
            "C" => Key4,
            "4" => Q,
            "5" => W,
            "6" => E,
            "D" => R,
            "7" => A,
            "8" => S,
            "9" => D,
            "E" => F,
            "A" => Z,
            "0" => X,
            "B" => C,
            "F" => V,
            _ => return None,
        })
    }
}