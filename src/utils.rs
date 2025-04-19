// utility function to exend the sign of a bit_count number to a 16 bit number
pub fn extend_sign(x: u16, bit_cnt: u8) -> u16 {
    if (x >> (bit_cnt - 1)) & 1 == 1 {
        x | (0xFFFF << bit_cnt)
    } else {
        x
    }
}

// utility function to extract the destination register from an instruction (11-9)
pub fn extract_dr(instruction: u16) -> usize {
    ((instruction >> 9) & 0x7) as usize
}

// utility function to extract the source register 1 from an instruction (8-6)
pub fn extract_sr1(instruction: u16) -> usize {
    ((instruction >> 6) & 0x7) as usize
}
