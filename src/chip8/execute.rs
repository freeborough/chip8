use super::{Chip8, SCREEN_WIDTH, SCREEN_HEIGHT};
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
                  println!("CLEAR");
                  self.clear_vram();
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
              self.is_running = false;
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
              self.v[x] = self.v[x].wrapping_add(nn);
            },
            // 0xANNN Set Index register to NNN
            0xA => {
              let nnn = get_nnn(instruction);
              println!("Set I to {nnn}");
              self.i = nnn;
            },
            // 0xDXYN Draw sprite that is N pixels tall in RAM at I, at location v[X], v[Y].
            0xD => {
              let x = get_x(instruction) as usize;
              let y = get_y(instruction) as usize;
              let n = get_n(instruction);

              // Get out X and Y coordinates.
              let x = (self.v[x] % SCREEN_WIDTH as u8) as usize;
              let y = (self.v[y] % SCREEN_HEIGHT as u8) as usize;

              for row in 0..n {
                let data = self.ram[self.i as usize + row as usize];
                let mut bit_mask: u8 = 0b10000000;

                for col in 0..8 {
                  // Get the current pixel from the byte.
                  let pixel = (data & bit_mask) >> (7 - col);

                  // Calculate the coordinates of this pixel.
                  let screen_x = x + col as usize;
                  let screen_y = y + row as usize;
                  let pixel_index = get_pixel_index(screen_x, screen_y);

                  let original_pixel = self.vram[pixel_index];
                  self.vram[pixel_index] = self.vram[pixel_index] ^ pixel;
                  
                  // If the pixel was flipped, set the carry flag.
                  if original_pixel != self.vram[pixel_index] {
                    self.v[15] = 1;
                  }

                  bit_mask = bit_mask >> 1;
                }
              }
            }
            _ => {}
          }

        // TODO: Have keyboard interaction to quit, as some programs may rely on "noop" for timing, etc.
        if instruction == 0 {
            self.is_running = false;
        }
    }
}
