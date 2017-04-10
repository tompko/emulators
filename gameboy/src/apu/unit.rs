use std::cmp;

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
    frequency: u8,
    counter: u8,
    fired: bool,
}

impl Timer {
    pub fn step(&mut self) {
        let (counter, fired) = self.counter.overflowing_add(self.frequency);
        self.counter = counter;
        self.fired = fired;
    }

    pub fn clock(&self) -> bool {
        self.fired
    }

    pub fn set_frequency(&mut self, frequency: u8) {
        self.frequency = frequency;
        self.counter = 0;
    }
}

#[derive(Default)]
pub struct LengthCounter {
    counter: u8,
    enabled: bool,
}

impl LengthCounter {
    pub fn step(&mut self) {
        if self.enabled {
            self.counter -= 1;
            if self.counter == 0 {
                self.enabled = false;
            }
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_counter(&mut self, val: u8) {
        self.counter = val;
    }
}

#[derive(Default)]
pub struct VolumeEnvelope {
    counter: u8,
    increment: bool,
}

impl VolumeEnvelope {
    pub fn step(&mut self) {
        if self.increment {
            self.counter = cmp::min(self.counter + 1, 15);
        } else {
            self.counter = self.counter.saturating_sub(1);
        }

    }

    pub fn set_volume(&mut self, volume: u8, increment: bool) {
        self.counter = volume;
        self.increment = increment;
    }

    pub fn get_volume(&self) -> u8 {
        self.counter
    }
}

#[derive(Default)]
pub struct SquareWave {
    duty: u8,
    state: u8,
}

impl SquareWave {
    pub fn step(&mut self) {
        self.state = (self.state + 1) % 8;
    }

    pub fn get_active(&self) -> bool {
        let waveform = match self.duty {
            0 => 0b00000001,
            1 => 0b10000001,
            2 => 0b10000111,
            3 => 0b01111110,
            _ => unreachable!(),
        };

        (waveform >> self.state) & 1 != 0
    }
}
