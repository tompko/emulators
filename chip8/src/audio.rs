use super::sdl2;
use super::sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(context: &sdl2::Sdl) -> Audio {
        let subsystem = context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let device = subsystem.open_playback(
            None,
            &desired_spec,
            |spec| {
                SquareWave{
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25
                }
            }
        ).unwrap();

        Audio{
            device: device,
        }
    }

    pub fn play(&mut self) {
        self.device.resume();
    }

    pub fn stop(&mut self) {
        self.device.pause();
    }
}


impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0...0.5 => self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}
