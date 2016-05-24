use super::memory::Memory;

pub struct Interconnect {
    pub mem: Memory
}

impl Interconnect {
    pub fn new() -> Interconnect {
        return Interconnect{
            mem: Memory::new()
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.load_rom(rom);
    }
}

