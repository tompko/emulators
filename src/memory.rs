pub const RAM_BYTES: usize = 4*1024;
pub const END_RESERVED: usize = 0x200;

pub struct Memory {
    ram: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        return Memory{
            ram: vec![0; RAM_BYTES]
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for i in 0..rom.len() {
            self.ram[i + END_RESERVED] = rom[i];
        }
    }
}
