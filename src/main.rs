mod cpu;
mod mem;
mod utils;

use std::{fs::File, io::{self, Read}, path::Path};
use clap::Parser;

use cpu::CPU;
use mem::Memory;

// CLI arguments definition
#[derive(Parser, Debug)]
#[command(name = "lc3emu", about = "a lightweigh LC-3 emulator written in Rust!")]
struct Args {
    program_path: String,
    #[arg(short, long)]
    verbose: bool
}

fn main() -> std::io::Result<()> {
    // parse CLI arguments
    let args = Args::parse();
    
    // instantiate a CPU and a Memory
    let mut cpu = CPU::new();
    let mut memory = Memory::new();
    
    // load the program into memory
    load_program(&mut memory, args.program_path)?;
    
    // untill we hit an HLT instruction keep stepping
    while cpu.running {
        // if verbose output is enable we print before and after state
        if args.verbose {
            println!("== instruction {:#X} ===", cpu.pc);
            println!("BEFORE\n {:?}", cpu);
        }
        // execute the instruction pointer by PC
        cpu.step(&mut memory);
        if args.verbose {
            println!("AFTER\n {:?}", cpu);
        }
    }

    Ok(())
}

fn load_program<P: AsRef<Path>>(mem: &mut Memory, path: P) -> io::Result<()> {
    // try to open the file
    let mut file = File::open(path)?;
    // allocate a buffer to hold the file's content
    let mut buffer = Vec::new();
    // read the file as a Vec<u8>
    file.read_to_end(&mut buffer)?;
    // the first 4 bytes of an LC3 program contains the starting PC addres
    let mut pc = 0;
    // read the file in 4 byte chunks and write those in the emulator's memory
    for chunk in buffer.chunks_exact(2) {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]); 
        if pc == 0 {
            pc = word;
        } else {
            mem.write_word(pc, word);
            pc += 1;
        }
    }
    Ok(())
}
