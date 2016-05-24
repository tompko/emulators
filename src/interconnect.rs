use super::sdl2;
use super::memory::Memory;
use super::graphics::Graphics;

pub struct Interconnect {
    pub mem: Memory,
    pub graphics: Graphics,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let context = sdl2::init().unwrap();

        return Interconnect{
            mem: Memory::new(),
            graphics: Graphics::new(&context),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.load_rom(rom);
    }
}
