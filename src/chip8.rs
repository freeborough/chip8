mod registers;
mod ram;
mod execute;
mod util;
pub mod debug;

/// The first accessable RAM address, leaving space for the font and historically the interpreter.
const START_ADDRESS: u16 = 0x200;

/// The size of the memory in bytes.
const RAM_SIZE: u16 = 4096;

/// The width of the screen in pixels.
const SCREEN_WIDTH: usize = 64;

/// The height of the screen in pixels.
const SCREEN_HEIGHT: usize = 32;

pub struct Chip8 {
    /// Program Counter: The address in ram of the next instruction to be executed.  Defaults to START_ADDRESS.
    pub pc: u16,

    /// Main memory where the program, data, and font are held.
    pub ram: Vec<u8>,

    /// VRAM, 0 = off, 1 = on.
    pub vram: Vec<u8>,

    /// The registers v0 to vF
    pub v: [u8; 16],

    /// The index register.
    pub i: u16,

    /// Whether or not the VM is running.
    is_running: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            pc: START_ADDRESS,
            ram: vec!(0; RAM_SIZE as usize),
            vram: vec!(0; SCREEN_WIDTH * SCREEN_HEIGHT),
            v: [0; 16],
            i: 0,
            is_running: false,
        }
    }
}