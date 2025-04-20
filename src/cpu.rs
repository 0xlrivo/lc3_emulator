use core::fmt;
use std::usize;

use crate::{mem::Memory, utils::{extend_sign, extract_dr, extract_offset9, extract_sr1}};

pub enum ConditionFlag {
    N = 0b100,  // Negative
    Z = 0b010,  // Zero
    P = 0b001,  // Positive
}

pub struct CPU {
    pub regs: [u16; 8],  // R0-R7
    pub pc: u16,
    pub cond: ConditionFlag,
    pub running: bool,
}

impl CPU {
    pub fn new() -> Self {
        CPU { regs: [0; 8], pc: 0x3000, cond: ConditionFlag::Z, running: true }
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
            0b0010 => self.op_ld(mem, instruction),
            0b1010 => self.op_ldi(mem, instruction),
            0b0110 => self.op_ldr(mem, instruction),
            0b1110 => self.op_lea(mem, instruction),
            0b0011 => self.op_st(mem, instruction),
            0b1011 => self.op_sti(mem, instruction),
            0b0111 => self.op_str(mem, instruction),
            0b1111 => self.op_trap(mem, instruction),
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
            let pc_offset = extend_sign(extract_offset9(instruction), 9);
            self.pc = self.pc.wrapping_add(pc_offset);
        }
    }

    // LD opcode -> loads the value of PC+offset9 into DR
    pub fn op_ld(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register
        let dr = extract_dr(instruction);
        // extract the offset and sign extend it
        let offset = extend_sign(extract_offset9(instruction), 9);
        // load memory word PC + offset into DR
        self.regs[dr] = mem.read_word(self.pc.wrapping_add(offset));
        // set condition flags
        self.update_flags(dr);
    }

    // LDI opcode -> basically a double LD
    pub fn op_ldi(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register
        let dr = extract_dr(instruction);
        // extract the offset and sign extend it
        let offset = extend_sign(extract_offset9(instruction), 9);
        // load 1
        let intermediate = mem.read_word(self.pc.wrapping_add(offset));
        // load 2
        self.regs[dr] = mem.read_word(intermediate);
        // update condition flags
        self.update_flags(dr);
    }

    // LDR
    pub fn op_ldr(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register
        let dr = extract_dr(instruction);
        // extract the base
        let base = (instruction >> 6) & 0x7; 
        // extract the offset6 and sign extend it
        let offset = extend_sign(instruction & 0x3F, 6);
        // load into DR the value at memory base + offset
        self.regs[dr] = mem.read_word(base.wrapping_add(offset));
        // update condition flags
        self.update_flags(dr);
    }

    // LEA (load effective address)
    pub fn op_lea(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register
        let dr = extract_dr(instruction);
        // extract the PCoffset9 and sign extend it
        let offset = extend_sign(extract_offset9(instruction), 9);
        // load into DR the value at memory word PC + offset
        self.regs[dr] = mem.read_word(self.pc.wrapping_add(offset));
        // update condition flags
        self.update_flags(dr);
    }

    // ST (store)
    pub fn op_st(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the source register
        let sr = extract_dr(instruction);
        // extract the PCoffset9 and sign extend it
        let offset = extend_sign(extract_offset9(instruction), 9);
        // store in memory the value of sr
        mem.write_word(self.pc.wrapping_add(offset), self.regs[sr]);
    }

    // STI (store indirect)
    pub fn op_sti(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the source register
        let sr = extract_dr(instruction);
        // extract the PCoffset9 and sign extend it
        let offset = extend_sign(extract_offset9(instruction), 9);
        // load the indirect address
        let intermediate = mem.read_word(self.pc.wrapping_add(offset));
        // store the value of sr into the memory word obtained in intermediate
        mem.write_word(intermediate, self.regs[sr]);
    }

    // STR (store base+offset)
    pub fn op_str(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the destination register
        let sr = extract_dr(instruction);
        // extract the base
        let base = (instruction >> 6) & 0x7; 
        // extract the offset6 and sign extend it
        let offset = extend_sign(instruction & 0x3F, 6);
        // store into base+offset the value of sr 
        mem.write_word(base.wrapping_add(offset), self.regs[sr]);
    }

    // TRAP opcode
    pub fn op_trap(&mut self, mem: &mut Memory, instruction: u16) {
        // extract the trap vector from the bits 0-7
        let trapvect = instruction & 0xFF;
        // execute the correct TRAP instruction
        match trapvect {
            0x25 => self.running = false,
            _ => unimplemented!(),
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

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== CPU ===")?;

        // print registers 4 per line
        for (i, reg) in self.regs.iter().enumerate() {
            write!(f, "R{}: 0x{:04X}\t", i, reg)?;
            if i == 3 { writeln!(f, "")?; }
        }

        // print program counter
        write!(f, "\nPC: 0x{:04X}\t", self.pc)?;

        // print conditional flag
        writeln!(f, "COND: {}", match self.cond {
            ConditionFlag::Z => "Z",
            ConditionFlag::N => "N",
            ConditionFlag::P => "P",
        })?;

        Ok(())
    }
}
