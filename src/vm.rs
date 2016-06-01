use super::interconnect::Interconnect;
use super::cpu::Cpu;

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
        self.cpu.run(&mut self.inter);
    }
}
