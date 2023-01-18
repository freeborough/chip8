use super::{Chip8, START_ADDRESS};

impl Chip8 {
    pub fn run(self: &mut Self) {
        self.is_running = true;

        loop {
            if self.is_running {
                self.cycle();
            } else {
                break;
            }
        }
    }

    pub fn fetch_instruction(self: &mut Self) -> u16 {
        let next_byte: u16 = self.ram[self.pc as usize + 1] as u16;
        self.ram[self.pc as usize] as u16 * 256 + next_byte
    }

    pub fn get_nnn(self: &mut Self, instruction: u16) -> u16 {
        (instruction << 4) >> 4
    }

    pub fn cycle(self: &mut Self) {
        let instruction = self.fetch_instruction();
        self.increment_pc();
        self.debug_instruction(instruction);
        
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
              let address = self.get_nnn(instruction);
              println!("JUMP: {:03X}", START_ADDRESS + address);
      
              self.pc = START_ADDRESS + address;
            },
            // 6 Set (Register)
            0x6 => {
              // TODO: Implement 6XNN Set Register
            }
            _ => {}
          }

        // TODO: Have keyboard interaction to quit, as some programs may rely on "noop" for timing, etc.
        if instruction == 0 {
            self.is_running = false;
        }
    }
}