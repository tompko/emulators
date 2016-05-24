use super::memory::Memory;
use super::graphics::Graphics;

pub struct Interconnect {
    pub mem: Memory,
    pub graphics: Graphics,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        return Interconnect{
            mem: Memory::new(),
            graphics: Graphics::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.load_rom(rom);
    }
}
