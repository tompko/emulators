use super::sdl2;
use super::sdl2::EventPump;
use super::sdl2::event::Event;

pub struct Input {
    event_pump: EventPump,
    pub quit: bool,
}

impl Input {
    pub fn new(context: &sdl2::Sdl) -> Input {
        return Input{
            event_pump: context.event_pump().unwrap(),
            quit: false,
        }
    }

    pub fn handle_input(&mut self) {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit{..} => {
                    self.quit = true;
                }
                _ => {}
            }

        }
    }

    pub fn key_pressed(&self, key_index: u8) -> bool {
        return false;
    }

    pub fn wait_for_keypress(&self) -> u8 {
        return 0;
    }
}
