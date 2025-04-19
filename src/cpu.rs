use std::usize;

use crate::{isa, mem::Memory, utils::{extend_sign, extract_dr, extract_sr1}};

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
        // fetch and increment program counter
        let instruction = mem.read_word(self.pc);
        self.pc = self.pc.wrapping_add(1);

        // extract the opcode (topmost 4 bits) from the instruction
        let opcode = instruction >> 12;
        
        // execute the opcode
        match opcode {
            0b0000 => self.op_br(mem, instruction),
            0b0001 => self.op_add(mem, instruction),
            0b0101 => self.op_and(mem, instruction),
            0b1001 => self.op_not(mem, instruction),
            _ => println!("Invalid Opcode"),
        }
    }
    
    // ADD opcode
    pub fn op_add(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register and source register 1
        let dr = extract_dr(instruction); 
        let sr1 = extract_sr1(instruction); 

        // extract the immediate flag, which differentiates the two ADDs
        let imm_flag = (instruction >> 5) & 0x1;

        let result = if imm_flag == 1 {
            let imm = extend_sign(instruction & 0x1F, 5);
            self.regs[sr1].wrapping_add(imm)
        } else {
            let sr2 = instruction & 0x7;
            self.regs[sr1].wrapping_add(self.regs[sr2 as usize])
        };

        // store the result in the destination register
        self.regs[dr] = result;
        // update flags based on dr state
        self.update_flags(dr);
    }
    
    // AND opcode
    pub fn op_and(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register and source register 1
        let dr = extract_dr(instruction); 
        let sr1 = extract_sr1(instruction); 

        // extract the immediate flag, which differentiates the two ADDs
        let imm_flag = (instruction >> 5) & 0x1;

        let result = if imm_flag == 1 {
            let imm = extend_sign(instruction & 0x1F, 5);
            self.regs[sr1] & imm
        } else {
            let sr2 = instruction & 0x7;
            self.regs[sr1] & self.regs[sr2 as usize]
        };

        // store the result in the destination register
        self.regs[dr] = result;
        // update flags based on dr state
        self.update_flags(dr);
    }
    
    // NOT opcode
    pub fn op_not(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register and source register 1
        let dr = extract_dr(instruction); 
        let sr1 = extract_sr1(instruction); 
        
        // invert every bit of sr1
        let result = !self.regs[sr1];
        
        // store the result in the destination register
        self.regs[dr] = result;
        // update flags based on dr state
        self.update_flags(dr);
    }

    // BR opcode
    pub fn op_br(&mut self, mem: &mut Memory, instruction: u16) {
        // extract n (11 th bit of instruction)
        let n = (instruction >> 11) & 0x1;
        // extract z (10 th bit of instruction)
        let z = (instruction >> 10) & 0x1;
        // extract p (9 th bit of instruction)
        let p = (instruction >> 9) & 0x1;
        
        // we branch if at least one of the conditions set are true
        let mut branch = false;
        match self.cond {
            ConditionFlag::N => if n == 1 { branch = true },
            ConditionFlag::Z => if z == 1 { branch = true },
            ConditionFlag::P => if p == 1 { branch = true },
        }

        // if we branch then extract the PCoffset9 and bitwise-OR it with PC 
        if branch {
            let pc_offset = instruction & 0x1FF;
            self.pc = self.pc | pc_offset;
        }
    }

    // update the conditional flags based on the lastest operation's result
    // stored in reg
    pub fn update_flags(&mut self, reg: usize) {
        // take the value stored in the provided register
        let val = self.regs[reg];
        // and update the condition flags based on his sign
        self.cond = if val == 0 {
            ConditionFlag::Z
        } else if (val >> 15) == 1 {
            // if the most signficat bit == 1 this is a negative number
            ConditionFlag::N
        } else {
            // if the most significant bit == 0 this is a positive number
            ConditionFlag::P
        }
    }
}
