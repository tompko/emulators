use super::time::SteadyTime;

const NSECS_PER_TICK: i64 = 16666666;

pub struct Timer {
    value: u8,
    nsec_elapsed: i64,
    last_time: SteadyTime,
}

impl Timer {
    pub fn new() -> Timer {
        Timer{
            value: 0,
            nsec_elapsed: 0,
            last_time: SteadyTime::now(),
        }
    }

    pub fn update(&mut self) {
        if self.value > 0 {
            let now = SteadyTime::now();
            let elapsed = now - self.last_time;
            if let Some(nsec_elapsed) = elapsed.num_nanoseconds() {
                self.nsec_elapsed += nsec_elapsed;
            } else {
                // We've waited 292 years for this timer to tick, assume we're done
                self.value = 0;
                self.nsec_elapsed = 0;
            }

            while self.nsec_elapsed > NSECS_PER_TICK {
                self.value = self.value.saturating_sub(1);
                self.nsec_elapsed -= NSECS_PER_TICK;
            }

            self.last_time = now;
        }
    }

    pub fn set(&mut self, val: u8) {
        self.value = val;
        self.nsec_elapsed = 0;
        self.last_time = SteadyTime::now();
    }

    pub fn get(&self) -> u8 {
        self.value
    }
}
