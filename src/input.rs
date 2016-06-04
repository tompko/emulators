use super::sdl2;
use super::sdl2::EventPump;
use super::sdl2::event::Event;
use super::sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: EventPump,
    quit: bool,
}

impl Input {
    pub fn new(context: &sdl2::Sdl) -> Input {
        Input{
            event_pump: context.event_pump().unwrap(),
            quit: false,
        }
    }

    pub fn handle_input(&mut self) {
        let events: Vec<Event> = self.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit{..} => self.quit = true,
                _ => {}
            }

        }
    }
}
