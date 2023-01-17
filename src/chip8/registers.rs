use super::{Chip8, START_ADDRESS};

impl Chip8 {
    /// Increments the Program Counter to the next instruction (which is 2 bytes along, as instructions are 16 bits).
    pub fn next_instruction(self: &mut Self) {
        self.pc = self.pc + 2;
    }

    /// Resets the Program Counter to the START_ADDRESS.
    pub fn reset_pc(self: &mut Self) {
        self.pc = START_ADDRESS;
    }
}