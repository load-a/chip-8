#[derive(Debug, Default, PartialEq)]
pub struct Instruction {
    pub nibble_a: u8,
    pub nibble_b: u8,
    pub nibble_c: u8,
    pub nibble_d: u8,
    pub opcode: u16,
    pub address: u16,
}

impl Instruction {
    pub fn new(bytes: [u8; 2], address: u16) -> Self {
        let nibble_a = (bytes[0] & 0b11110000) >> 4;
        let nibble_b = bytes[0] & 0b00001111;
        let nibble_c = (bytes[1] & 0b11110000) >> 4;
        let nibble_d = bytes[1] & 0b00001111;

        Self { nibble_a: nibble_a, 
            nibble_b: nibble_b, 
            nibble_c: nibble_c, 
            nibble_d: nibble_d,
            opcode: u16::from_be_bytes(bytes), 
            address: address,
        }
    }

    pub fn category(&self) -> u8 {
        self.nibble_a
    }

    pub fn x(&self) -> u8 {
        self.nibble_b
    }

    pub fn y(&self) -> u8 {
        self.nibble_c
    }

    pub fn n(&self) -> u8 {
        self.nibble_d
    }

    pub fn nn(&self) -> u8 {
        (self.nibble_c << 4) | self.nibble_d
    }

    pub fn nnn(&self) -> u16 {
        u16::from_be_bytes([self.nibble_b, self.nn()])
    }
}