use super::{Chip8};

impl Chip8 {
    pub fn debug(self: &mut Self) {
        println!("PC: {}", self.pc);
    }

    pub fn debug_instruction(self: &mut Self, instruction: u16) {
        println!("{}", self.format_instruction(instruction));
    }
}