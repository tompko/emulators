pub const RAM_BYTES: usize = 2*1024;

pub struct Memory {
    ram: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            ram: vec![0; RAM_BYTES]
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.ram[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}
