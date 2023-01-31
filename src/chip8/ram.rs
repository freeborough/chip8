use std::fs;

use super::{Chip8, RAM_SIZE, START_ADDRESS, SCREEN_WIDTH, SCREEN_HEIGHT};

impl Chip8 {
    /// Clears the RAM, setting everything to zero.
    pub fn clear_ram(self: &mut Self) {
        self.ram.clear();
        self.ram.resize(RAM_SIZE as usize, 0);
    }

    /// Clears the VRAM, setting everything to zero.
    pub fn clear_vram(self: &mut Self) {
        self.vram.clear();
        self.vram.resize(SCREEN_WIDTH * SCREEN_HEIGHT, 0);
    }

    /// Loads the data (e.g. a "rom") from the specified file into our VM's ram.
    /// NOTE: This will clear the RAM and reset the Program Counter.
    pub fn load(self: &mut Self, path: &str) {
        self.clear_ram();
        self.reset_pc();
        let buffer = fs::read(path).unwrap();
        self.ram[START_ADDRESS as usize..][..buffer.len()].copy_from_slice(&buffer);
    }
}