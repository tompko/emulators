use super::interconnect::Interconnect;
use super::cpu::Cpu;
use super::time::{Duration, SteadyTime};
use std::thread;

const NSECS_PER_FRAME: i64 = 1000000000 / 500;

pub struct VM {
    inter: Interconnect,
    cpu: Cpu
}

impl VM {
    pub fn new() -> VM {
        let inter = Interconnect::new();
        let cpu = Cpu::new();
        VM{
            inter: inter,
            cpu: cpu,
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.inter.load_rom(rom);
    }

    pub fn run(&mut self) {
        let mut start_time = SteadyTime::now();
        let mut nsecs_elapsed = 0;

        loop {
            self.cpu.step(&mut self.inter);

            self.inter.graphics.render();
            self.inter.input.handle_input();

            if self.inter.input.quit {
                break;
            }

            let mut now = SteadyTime::now();
            let mut duration_elapsed = now - start_time;
            nsecs_elapsed += duration_elapsed.num_nanoseconds().expect("Loop took too long");
            start_time = now;

            let nsecs_to_sleep = NSECS_PER_FRAME - nsecs_elapsed;
            if nsecs_to_sleep > 0 {
                let sleep_duration = Duration::nanoseconds(nsecs_to_sleep);
                thread::sleep(sleep_duration.to_std().unwrap());
            }

            now = SteadyTime::now();
            duration_elapsed = now - start_time;
            nsecs_elapsed += duration_elapsed.num_nanoseconds().expect("Loop took too long");
            while nsecs_elapsed > NSECS_PER_FRAME {
                // Ignore fully missed frames
                nsecs_elapsed -= NSECS_PER_FRAME;
            }
        }
    }
}
