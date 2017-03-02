use super::sdl2;
use super::sdl2::EventPump;
use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: EventPump,
    keys: [bool; 16],
    pub quit: bool,
}

impl Input {
    pub fn new(context: &sdl2::Sdl) -> Input {
        Input{
            event_pump: context.event_pump().unwrap(),
            keys: [false;16],
            quit: false,
        }
    }

    pub fn handle_input(&mut self) {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit{..} => self.quit = true,

                Event::KeyDown{keycode: Some(Keycode::Num0), ..} => self.keys[0x0] = true,
                Event::KeyDown{keycode: Some(Keycode::Num1), ..} => self.keys[0x1] = true,
                Event::KeyDown{keycode: Some(Keycode::Num2), ..} => self.keys[0x2] = true,
                Event::KeyDown{keycode: Some(Keycode::Num3), ..} => self.keys[0x3] = true,
                Event::KeyDown{keycode: Some(Keycode::Num4), ..} => self.keys[0x4] = true,
                Event::KeyDown{keycode: Some(Keycode::Num5), ..} => self.keys[0x5] = true,
                Event::KeyDown{keycode: Some(Keycode::Num6), ..} => self.keys[0x6] = true,
                Event::KeyDown{keycode: Some(Keycode::Num7), ..} => self.keys[0x7] = true,
                Event::KeyDown{keycode: Some(Keycode::Num8), ..} => self.keys[0x8] = true,
                Event::KeyDown{keycode: Some(Keycode::Num9), ..} => self.keys[0x9] = true,
                Event::KeyDown{keycode: Some(Keycode::A), ..} => self.keys[0xa] = true,
                Event::KeyDown{keycode: Some(Keycode::B), ..} => self.keys[0xb] = true,
                Event::KeyDown{keycode: Some(Keycode::C), ..} => self.keys[0xc] = true,
                Event::KeyDown{keycode: Some(Keycode::D), ..} => self.keys[0xd] = true,
                Event::KeyDown{keycode: Some(Keycode::E), ..} => self.keys[0xe] = true,
                Event::KeyDown{keycode: Some(Keycode::F), ..} => self.keys[0xf] = true,

                Event::KeyUp{keycode: Some(Keycode::Num0), ..} => self.keys[0x0] = false,
                Event::KeyUp{keycode: Some(Keycode::Num1), ..} => self.keys[0x1] = false,
                Event::KeyUp{keycode: Some(Keycode::Num2), ..} => self.keys[0x2] = false,
                Event::KeyUp{keycode: Some(Keycode::Num3), ..} => self.keys[0x3] = false,
                Event::KeyUp{keycode: Some(Keycode::Num4), ..} => self.keys[0x4] = false,
                Event::KeyUp{keycode: Some(Keycode::Num5), ..} => self.keys[0x5] = false,
                Event::KeyUp{keycode: Some(Keycode::Num6), ..} => self.keys[0x6] = false,
                Event::KeyUp{keycode: Some(Keycode::Num7), ..} => self.keys[0x7] = false,
                Event::KeyUp{keycode: Some(Keycode::Num8), ..} => self.keys[0x8] = false,
                Event::KeyUp{keycode: Some(Keycode::Num9), ..} => self.keys[0x9] = false,
                Event::KeyUp{keycode: Some(Keycode::A), ..} => self.keys[0xa] = false,
                Event::KeyUp{keycode: Some(Keycode::B), ..} => self.keys[0xb] = false,
                Event::KeyUp{keycode: Some(Keycode::C), ..} => self.keys[0xc] = false,
                Event::KeyUp{keycode: Some(Keycode::D), ..} => self.keys[0xd] = false,
                Event::KeyUp{keycode: Some(Keycode::E), ..} => self.keys[0xe] = false,
                Event::KeyUp{keycode: Some(Keycode::F), ..} => self.keys[0xf] = false,
                _ => {}
            }

        }
    }

    pub fn key_pressed(&self, key_index: u8) -> bool {
        self.keys[key_index as usize]
    }

    pub fn any_key_pressed(&self) -> Option<u8> {
        for i in 0..16 {
            if self.keys[i] {
                return Some(i as u8);
            }
        }

        None
    }
}
