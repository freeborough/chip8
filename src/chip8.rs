mod registers;
mod ram;
mod execute;
pub mod util;
mod debug;

/// The first accessable RAM address, leaving space for the font and historically the interpreter.
const START_ADDRESS: u16 = 0x200;

/// The size of the memory in bytes.
const RAM_SIZE: u16 = 4096;

pub struct Chip8 {
    /// Program Counter: The address in ram of the next instruction to be executed.  Defaults to START_ADDRESS.
    pub pc: u16,

    /// Main memory where the program, data, and font are held.
    pub ram: Vec<u8>,

    /// The registers v0 to vF
    pub v: [u8; 0xF],

    /// Whether or not the VM is running.
    is_running: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            pc: START_ADDRESS,
            ram: vec!(0; RAM_SIZE as usize),
            v: [0; 0xF],
            is_running: false,
        }
    }
}