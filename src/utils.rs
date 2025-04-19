// utility function to exend the sign of a bit_count number to a 16 bit number
pub fn extend_sign(x: u16, bit_cnt: u8) -> u16 {
    if (x >> (bit_cnt - 1)) & 1 == 1 {
        x | (0xFFFF < bit_count)
    } else {
        x
    }
}
