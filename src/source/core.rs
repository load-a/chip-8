use std::fs;
use std::env;

pub struct Source {
    pub raw: String,
    pub bytes: Vec<u8>,
}

impl Source {
    // #[cfg(not(test))]
    pub fn new() -> Source {
        let args: Vec<String> = env::args().collect();

        if args.len() != 2 {
            eprintln!("Usage {} <filename>", args[0]);
            std::process::exit(1);
        }

        let filename = &args[1];
        let raw = fs::read_to_string(filename).unwrap();
        let bytes: Vec<u8> = raw
            .split_whitespace()
            .flat_map(|word| {
                let byte = u16::from_str_radix(word, 16).unwrap();
                let high_byte = (byte >> 8) as u8;
                let low_byte = (byte & 0xFF) as u8;

                vec![high_byte, low_byte]
            })
            .collect();

        Self {
            raw: raw,
            bytes: bytes,
        }
    }

    pub fn print_bytes(&self) {
        for byte in &self.bytes {
            println!("{}", byte.to_string())
        }
    }

    // #[cfg(test)]
    // pub fn new(args: [String; 2]) {
    //     let args: Vec<String> = args.to_vec();
    // }
}
