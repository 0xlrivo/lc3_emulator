pub enum Opcode {
    ADD = 0b0001,
    AND = 0b0101,
    BR = 0b0000,
    JMP_RET = 0b1100,
    JSR = 0b0100,
    LD = 0b0010,
    LDI = 0b1010,
    LDR = 0b0110,
    LEA = 0b1110,
    NOT = 0b1001,
    ST = 0b0011,
    STI = 0b1011,
    SRT = 0b0111,
    TRAP = 0b1111,
    RESERVED = 0b1101
}

pub fn decode_and_execute(instruction: u16) {
    unimplemented!();
}
