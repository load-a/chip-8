use crate::chip8::{core::Chip8, flag_register::FlagRegister};
use crate::address::Address;
use crate::instruction::Instruction;

pub trait Decoder {
    fn decode(&mut self, instruction: Instruction);
    fn category_zero(&mut self, instruction: Instruction);
    fn category_one(&mut self, instruction: Instruction);
    fn category_two(&mut self, instruction: Instruction);
    fn category_three(&mut self, instruction: Instruction);
    fn category_four(&mut self, instruction: Instruction);
    fn category_five(&mut self, instruction: Instruction);
    fn category_six(&mut self, instruction: Instruction);
    fn category_seven(&mut self, instruction: Instruction);
    fn category_eight(&mut self, instruction: Instruction);
    fn category_nine(&mut self, instruction: Instruction);
    fn category_ten(&mut self, instruction: Instruction);
    fn category_eleven(&mut self, instruction: Instruction);
    fn category_twelve(&mut self, instruction: Instruction);
    fn category_thirteen(&mut self, instruction: Instruction);
    fn category_fourteen(&mut self, instruction: Instruction);
    fn category_fifteen(&mut self, instruction: Instruction);
}

impl Decoder for Chip8 {
    fn decode(&mut self, instruction: Instruction) {
        match instruction.category() {
            0x0 => self.category_zero(instruction),
            0x1 => self.category_one(instruction),
            0x2 => self.category_two(instruction),
            0x3 => self.category_three(instruction),
            0x4 => self.category_four(instruction),
            0x5 => self.category_five(instruction),
            0x6 => self.category_six(instruction),
            0x7 => self.category_seven(instruction),
            0x8 => self.category_eight(instruction),
            0x9 => self.category_nine(instruction),
            0xA => self.category_ten(instruction),
            0xB => self.category_eleven(instruction),
            0xC => self.category_twelve(instruction),
            0xD => self.category_thirteen(instruction),
            0xE => self.category_fourteen(instruction),
            0xF => self.category_fifteen(instruction),
            _ => ()
        }
    }

    fn category_zero(&mut self, instruction: Instruction) {
        match instruction.nibble_c {
            0xE => {
                if instruction.nibble_d == 0xE {
                    // Return from subroutine
                } else {
                    // self.screen.color(0x000000);
                    self.screen.blackout();
                }
            },
            _ => println!("Instruction: SysAddress 0NNN is not implemented on modern interpreters.")
        }
    }

    fn category_one(&mut self, instruction: Instruction) {
        // Jump NNN
        self.program_counter = instruction.nnn() 
    }

    fn category_two(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_three(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_four(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_five(&mut self, instruction: Instruction) {
        todo!()
    }

    fn category_six(&mut self, instruction: Instruction) {
        // Set VX to NN
        self.register[instruction.x() as usize] = instruction.nn();
    }

    fn category_seven(&mut self, instruction: Instruction) {
        let x_register = instruction.x() as usize;

        self.register[x_register] = self.register[x_register].wrapping_add(instruction.nn());
    }

    fn category_eight(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_nine(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_ten(&mut self, instruction: Instruction) {
        self.index_register = instruction.nnn();
    }
    fn category_eleven(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_twelve(&mut self, instruction: Instruction) {
        todo!()
    }

    fn category_thirteen(&mut self, instruction: Instruction) {
        // Get the screen coordinate x from Register X
        let sprite_x = self.register[instruction.x() as usize];

        // Get the screen coordinate y from Register Y
        let sprite_y = self.register[instruction.y() as usize];

        // Get the sprite height from [N]
        let sprite_height = instruction.n();

        // Unset the Flag Register
        self.reset_flag_register();

        // N-times, do the following:
        for row in 0..sprite_height {
            // Don't wrap
            if row > 32 { break }

            //  Read the sprite row
            let sprite = self.memory[(self.index_register + (row as u16)) as usize];

            //  Shift it to match the location (sending extra bits to a second buffer)
            let shift = sprite_x % 8;
            let acnhor_byte = sprite >> shift;
            let overflow_byte = sprite << (8 - shift);

            //  XOR it to the bytes starting at (x, y + N) [do this twice if shifted]
            for bit in 0..8 {
                
            }

            //  STOP at screen boundaries
            //  If any pixels were turned off, set the Flag Register
        }
    }

    fn category_fourteen(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_fifteen(&mut self, instruction: Instruction) {
        todo!()
    }
}
