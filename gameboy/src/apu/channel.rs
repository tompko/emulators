use apu::unit::*;

pub struct Channel1 {
    sweep: Sweep,
    timer: Timer,
    duty: SquareWave,
    length: LengthCounter,
    envelope: VolumeEnvelope,
}

impl Channel1 {
    pub fn new() -> Self {
        Channel1 {
            sweep: Sweep::default(),
            timer: Timer::default(),
            duty: SquareWave::default(),
            length: LengthCounter::default(),
            envelope: VolumeEnvelope::default(),
        }
    }
}

impl AudioStep for Channel1 {
    fn step(&mut self, frame_seq: &FrameSequencer) {
        self.sweep.step(frame_seq);
        self.timer.step(frame_seq);
        self.duty.step(frame_seq);
        self.length.step(frame_seq);
        self.envelope.step(frame_seq);
    }
}

impl AudioSource for Channel1 {
    fn generate(&mut self) -> u16 {
        let sweep = self.sweep.generate();
        let timer = self.timer.process(sweep);
        let duty = self.duty.process(timer);
        let length = self.length.process(duty);
        let volume = self.envelope.process(length);

        volume
    }
}
