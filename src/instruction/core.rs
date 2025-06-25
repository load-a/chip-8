#[derive(Clone, Debug, Default, PartialEq)]
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

    // pub fn debug(&self) -> String {
    //     let category = match self.nibble_a {
    //         0x0 => "Meta",
    //         0x1 => "Jump",
    //         0x2 => "Call",
    //         0x3 => "VX_Inequal",
    //         0x4 => "VX_Equal",
    //         0x5 => "VX_VY_Inequal",
    //         0x6 => "Assign_Literal",
    //         0x7 => "Add_Assign",
    //         0x8 => "Math_and_Logic",
    //         0x9 => "VX_VY_Equal",
    //         0xA => "Set_Index",
    //         0xB => "Jump_with_Offset",
    //         0xC => "Random_Number",
    //         0xD => "Draw",
    //         0xE => "Input_State",
    //         0xF => "Output_and_State",
    //         _ => "UNDEFINED",
    //     }
    // }
}

enum Category {
    System,
    Jump,
    Call,
    SkipInequal,

    SkipEqual,
    SkipRegistersInequal,
    LoadImmediate,
    AddImmediate,

    ArithmeticLogic,
    SkipRegistersEqual,
    LoadIndex,
    JumpOffset,

    Random,
    Draw,
    SkipInput,
    Misc,
}