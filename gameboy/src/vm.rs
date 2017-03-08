use interconnect::Interconnect;
use cpu::Cpu;

pub struct VM<T: Interconnect> {
    cpu: Cpu,
    inter: T,
}

impl<T: Interconnect> VM<T> {
    pub fn new(interconnect: T) -> VM<T> {
        let cpu = Cpu::new();
        VM {
            inter: interconnect,
            cpu: cpu,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.step();
        }
    }

    pub fn step(&mut self) -> u16 {
        self.cpu.step(&mut self.inter)
    }

    pub fn get_children(self) -> (Cpu, T) {
        (self.cpu, self.inter)
    }
}
