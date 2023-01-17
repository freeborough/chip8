use super::Chip8;

impl Chip8 {
    pub fn debug(self: Self) {
        println!("PC: {}", self.pc);
    }
}