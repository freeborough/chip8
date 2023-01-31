use super::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub fn format_instruction(instruction: u16) -> String {
    format!("{:04X}", instruction)
}


// _nnn
// 0FFF
pub fn get_nnn(instruction: u16) -> u16 {
    instruction & 0x0FFF
}

// __nn
// 00FF
pub fn get_nn(instruction: u16) -> u8 {
    (instruction & 0x00FF) as u8
}

// ___n
// 000F
pub fn get_n(instruction: u16) -> u8 {
    (instruction & 0x000F) as u8
}

// _x__
// 0F00 >> 8
// 000F
pub fn get_x(instruction: u16) -> u8 {
    ((instruction & 0x0F00) >> 8) as u8
}

// __y_
// 00F0 >> 4
pub fn get_y(instruction: u16) -> u8 {
    ((instruction & 0x00F0) >> 4) as u8
}

pub fn get_pixel_index(x: usize, y: usize) -> usize {
    (y * SCREEN_WIDTH) + x
}