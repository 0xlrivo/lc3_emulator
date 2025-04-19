use crate::mem::Memory;
use crate::isa::{self, Opcode};

pub enum ConditionFlag {
    N = 0b100,  // Negative
    Z = 0b010,  // Zero
    P = 0b001,  // Positive
}

pub struct CPU {
    pub regs: [u16; 8],  // R0-R7
    pub pc: u16,
    pub cond: ConditionFlag,
}

impl CPU {
    pub fn new() -> Self {
        CPU { regs: [0; 8], pc: 0x3000, cond: ConditionFlag::Z }
    }

    // fetch-decode-execute loop implementation
    pub fn step(&mut self, mem: &mut Memory) {
        // fetch
        let instruction = mem.read(self.pc);
        
        // decode and execute
        isa::decode_and_execute(instruction);

        // increment program counter
        self.pc += 1;
    }
}
