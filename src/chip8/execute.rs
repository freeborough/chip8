use super::{Chip8};
use super::debug::output_instruction;
use super::util::*;

impl Chip8 {
    pub fn run(self: &mut Self) {
        self.is_running = true;

        loop {
            match self.is_running {
              true => self.cycle(),
              false => break,
            }
        }
    }

    pub fn fetch_instruction(self: &mut Self) -> u16 {
        let next_byte: u16 = self.ram[self.pc as usize + 1] as u16;
        self.ram[self.pc as usize] as u16 * 256 + next_byte
    }

    pub fn cycle(self: &mut Self) {
        let instruction = self.fetch_instruction();
        self.increment_pc();
        output_instruction(instruction);
        
        let instruction_group = instruction >> 12;

        match instruction_group {
            // 0: Clear Screen, and more.
            0x0 => {
              match instruction {
                // 00E0: Clear Screen
                0x00E0 => {
                  // TODO: Actually clear the screen.
                  println!("Clear Screen");
                },
                _ => {}
              }
            },
            // 0x1NNN Jump to NNN
            0x1 => {
              let address = get_nnn(instruction);
              println!("JUMP: {:03X}", address);
      
              self.pc = address;
              // DEBUG:
              std::process::exit(0);
            },
            // 6 Set (Register)
            // 6XNN Set register X to NN
            0x6 => {
              let x = get_x(instruction) as usize;
              let nn = get_nn(instruction);
              println!("SET v{x} to {nn}");
              self.v[x] = nn;
            },
            // 0x7XNN Add NN to register X
            0x7 => {
              let x = get_x(instruction) as usize;
              let nn = get_nn(instruction);
              println!("ADD {nn} to v{x}");
              self.v[x] = add_u8(self.v[x], nn);
            },
            _ => {}
          }

        // TODO: Have keyboard interaction to quit, as some programs may rely on "noop" for timing, etc.
        if instruction == 0 {
            self.is_running = false;
        }
    }
}
