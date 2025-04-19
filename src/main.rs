mod cpu;
mod mem;
mod isa;
mod utils;

use cpu::CPU;
use mem::Memory;

fn main() {
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    
    println!("Starting execution from {:#x}", cpu.pc);

    loop {
        cpu.step(&mut memory);
    }
}
