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
        // Get sprite data (screen position, height)
        let x = self.register[instruction.x() as usize] as usize;
        let y = self.register[instruction.y() as usize] as usize;
        let height = instruction.n() as usize;

        // Reset flag
        self.reset_flag_register();

        // Do this for every row
        for row in 0..height {
            // Get the sprite for this row
            let sprite = self.memory[(self.index_register + row as u16) as usize];

            // For every bit in this sprite, do this:
            for bit in 0..8 {
                // Get the x and y for the current pixel
                let pixel_x = (x + bit) % 64;
                let pixel_y = (y + row) % 32;
                // Calculate this pixel's index in the Frame
                let idx = pixel_x + pixel_y * 64;

                // Get the current pixel color from the sprite
                let sprite_pixel = (sprite >> (7 - bit)) & 1;
                // Get the old pixel color and convert it to a bit
                let old_pixel = if self.screen.frame[idx] == self.screen.pixel_on { 1 } else { 0 };
                // XOR the old and new pixel
                let new_pixel = old_pixel ^ sprite_pixel;

                // If the old pixel was turned off, set the flag
                if old_pixel == 1 && new_pixel == 0 {
                    self.set_flag_register();
                }

                // Convert the new pixel into its actual color
                self.screen.frame[idx] = if new_pixel == 1 {
                    self.screen.pixel_on
                } else {
                    self.screen.pixel_off
                };
            }
        }
    }

    fn category_fourteen(&mut self, instruction: Instruction) {
        todo!()
    }
    fn category_fifteen(&mut self, instruction: Instruction) {
        todo!()
    }
}
