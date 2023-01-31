use super::Chip8;
use super::util::format_instruction;

impl Chip8 {
    #[allow(dead_code)]
    pub fn debug(self: &Self) {
        println!("PC: {}", self.pc);
    }
}

/// Outputs instruction to console in readable format.
pub fn output_instruction(instruction: u16) {
    println!("{}", format_instruction(instruction));
}