use std::cmp;

pub trait AudioStep {
    fn step(&mut self, frame_seq: &FrameSequencer);
}

pub trait AudioSource: AudioStep {
    fn generate(&mut self) -> u16;
}

pub trait AudioProcess: AudioStep {
    fn process(&mut self, audio: u16) -> u16;
}

#[derive(Default)]
pub struct FrameSequencer {
    state: u8,
}

impl FrameSequencer {
    pub fn step(&mut self) {
        self.state = (self.state + 1) % 8;
    }

    pub fn length_clock(&self) -> bool {
        self.state % 2 == 0
    }

    pub fn volume_clock(&self) -> bool {
        self.state == 7
    }

    pub fn sweep_clock(&self) -> bool {
        self.state == 2 || self.state == 6
    }
}

#[derive(Default)]
pub struct Timer {
    frequency: u32,
    counter: u32,
    fired: bool,
}

impl Timer {
    pub fn clock(&self) -> bool {
        self.fired
    }

    pub fn set_frequency(&mut self, frequency: u16) {
        self.frequency = frequency as u32;
        self.counter = 0;
    }
}

impl AudioStep for Timer {
    fn step(&mut self, _: &FrameSequencer) {
        self.fired = false;
        self.counter += self.frequency;

        if self.counter > 0x00400000 {
            self.counter -= 0x00400000;
            self.fired = true;
        }
    }
}

impl AudioSource for Timer {
    fn generate(&mut self) -> u16 {
        if self.fired {
            1
        } else {
            0
        }
    }
}

impl AudioProcess for Timer {
    fn process(&mut self, audio: u16) -> u16 {
        if audio != 0 {
            self.set_frequency(audio);
        }

        self.generate()
    }
}

#[derive(Default)]
pub struct LengthCounter {
    counter: u8,
    enabled: bool,
}

impl LengthCounter {
    pub fn set_counter(&mut self, val: u8) {
        self.counter = val;
    }
}

impl AudioStep for LengthCounter {
    fn step(&mut self, frame_seq: &FrameSequencer) {
        if self.enabled && frame_seq.length_clock() {
            self.counter -= 1;
            if self.counter == 0 {
                self.enabled = false;
            }
        }
    }
}

impl AudioProcess for LengthCounter {
    fn process(&mut self, audio: u16) -> u16 {
        if self.enabled && audio != 0 {
            1
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct VolumeEnvelope {
    counter: u8,
    increment: bool,
}

impl VolumeEnvelope {
    pub fn set_volume(&mut self, volume: u8, increment: bool) {
        self.counter = volume;
        self.increment = increment;
    }

}

impl AudioStep for VolumeEnvelope {
    fn step(&mut self, frame_seq: &FrameSequencer) {
        if frame_seq.volume_clock() {
            if self.increment {
                self.counter = cmp::min(self.counter + 1, 15);
            } else {
                self.counter = self.counter.saturating_sub(1);
            }
        }
    }
}

impl AudioProcess for VolumeEnvelope {
    fn process(&mut self, audio: u16) -> u16 {
        if audio != 0 {
            self.counter as u16
        } else {
            0
        }
    }
}

#[derive(Default)]
pub struct SquareWave {
    duty: u8,
    state: u8,
}

impl AudioStep for SquareWave {
    fn step(&mut self, _: &FrameSequencer) {
    }
}

impl AudioProcess for SquareWave {
    fn process(&mut self, audio: u16) -> u16 {
        if audio != 0 {
            self.state = (self.state + 1) % 8;
        }

        let waveform = match self.duty {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => unreachable!(),
        };

        (waveform >> self.state)
    }
}

#[derive(Default)]
pub struct Sweep {
    // TODO
}

impl AudioStep for Sweep {
    fn step(&mut self, _: &FrameSequencer) {
        // TODO
    }
}

impl AudioSource for Sweep {
    fn generate(&mut self) -> u16 {
        // TODO
        0
    }
}
