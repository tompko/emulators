use super::sdl2;
use super::memory::Memory;
use super::graphics::Graphics;
use super::input::Input;
use super::audio::Audio;


pub struct Interconnect {
    pub mem: Memory,
    pub graphics: Graphics,
    pub input: Input,
    pub audio: Audio,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let context = sdl2::init().unwrap();

        Interconnect{
            mem: Memory::new(),
            graphics: Graphics::new(&context),
            input: Input::new(&context),
            audio: Audio::new(&context),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.mem.load_rom(rom);
    }
}
