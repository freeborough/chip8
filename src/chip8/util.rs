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

// _x__
// 0F00 >> 8
// 000F
pub fn get_x(instruction: u16) -> u8 {
    ((instruction & 0x0F00) >> 8) as u8
}

pub fn add_u8(a: u8, b: u8) -> u8 {
    let (answer, _overflowed) = a.overflowing_add(b);
    answer
}