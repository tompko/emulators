use super::sdl2;
use super::memory::Memory;
use super::graphics::Graphics;
use super::input::Input;


pub struct Interconnect {
    pub mem: Memory,
    pub graphics: Graphics,
    pub input: Input,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let context = sdl2::init().unwrap();

        Interconnect{
            mem: Memory::new(),
            graphics: Graphics::new(&context),
            input: Input::new(&context),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.load_rom(rom);
    }
}
