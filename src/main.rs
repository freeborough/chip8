mod chip8;

use chip8::Chip8;
use chip8::debug::*;

fn main() {
  println!("Chip8 v0.1");

  let mut vm = Chip8::new();

  vm.load("./roms/IBM Logo.ch8");
  vm.run();
  output_vram(vm);
}