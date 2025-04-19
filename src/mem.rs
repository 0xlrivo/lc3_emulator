// start address for the Trap Vector Table
const TVT: u16 = 0x0000;
// start address for the Interrupt Vector Table
const IVT: u16 = 0x0100;
// start address for OS and Supervisor Stack
const OS_ST: u16 = 0x0200;
// start address for the user-program memory region
const USER_PRG: u16 = 0x3000;
// start addres for the memory mapped devices region
const MMDEV: u16 = 0xFE00;

pub struct Memory {
    pub mem: [u16; u16::MAX as usize + 1], // 2^16 memory locations
}

impl Memory {
    pub fn new() -> Self {
        Memory {mem: [0; u16::MAX as usize + 1]}
    }

    // allows to read a memory word given his address
    pub fn read_word(&self, addr: u16) -> u16 {
        self.mem[addr as usize]
    }

    // allows to write a word in a given memory location
    pub fn write_word(&mut self, addr: u16, val: u16) {
        self.mem[addr as usize] = val;
    }
}
