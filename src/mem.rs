pub struct Memory {
    pub mem: [u16; u16::MAX as usize + 1], // 2^16 memory locations
}

impl Memory {
    pub fn new() -> Self {
        Memory {mem: [0; u16::MAX as usize + 1]}
    }

    // allows to read a memory word given his address
    pub fn read(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    // allows to write a word in a given memory location
    pub fn write(&mut self, addr: u16, val: u16) {
        self.mem[addr as usize] = val;
    }
}
