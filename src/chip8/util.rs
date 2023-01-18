pub fn format_instruction(instruction: u16) -> String {
    format!("{:04X}", instruction)
}

pub fn get_nnn(instruction: u16) -> u16 {
    (instruction << 4) >> 4
}