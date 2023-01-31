use std::fs;

fn main() {
  println!("Chip 8 Beginnings Part 2");

  // Program Counter: The current instruction being executed.
  let mut pc: u16 = 0;

  // Registers: v0 to vF
  let mut v: [u8; 0xF] = [0; 0xF];

  let program = fs::read("/home/andy/dev/learning/rust/chip8/roms/IBM Logo.ch8").unwrap();
  
  /*
  00E0 (clear screen)
  ** 1NNN (jump)
  ** 6XNN (set register VX)
  **7XNN (add value to register VX)
  ANNN (set index register I)
  DXYN (display/draw)
  */

  // Main execution loop.
  loop {
    let instruction = fetch_next_instruction(&program, pc);
    let instruction_group = instruction >> 12;
    print_instruction(instruction_group);

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
      // 1 Jump
      0x1 => {
        let address = get_nnn(instruction);
        println!("JUMP: {:03X}", address);

        pc = address;
      },
      // 6 Set (Register)
      0x6 => {
        // TODO: Implement 6XNN Set Register
      }
      _ => {}
    }

    // Increment Program Counter to next execution.
    pc = pc + 2;
    if pc as usize >= program.len() {
      break;
    }
  }
}


fn fetch_next_instruction(program: &Vec<u8>, pc: u16) -> u16 {
  let next_byte: u16 = program[pc as usize + 1] as u16;
  program[pc as usize] as u16 * 256 + next_byte
}

fn print_instruction(instruction: u16) {
  println!("{:04X}", instruction);
}

fn get_nnn(instruction: u16) -> u16 {
  (instruction << 4) >> 4
}