use super::{Chip8, SCREEN_WIDTH, SCREEN_HEIGHT};
use super::util::{format_instruction, get_pixel_index};

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
// 
//
pub fn output_vram(vm: Chip8) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let pixel_index = get_pixel_index(x, y);
            if vm.vram[pixel_index] == 0 {
                print!(" ");
            } else {
                print!("█");
            }
        }
        println!("");
    }
}