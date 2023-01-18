use super::{Chip8};

// TODO: Should these be impl Chip8 or just general functions?
impl Chip8 {
    pub fn format_instruction(self: &mut Self, instruction: u16) -> String {
        format!("{:04X}", instruction)
    }
}
