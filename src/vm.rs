use super::interconnect::Interconnect;
use super::cpu::Cpu;

pub struct VM {
    inter: Interconnect,
    cpu: Cpu
}

impl VM {
    pub fn new(cart_rom: Box<[u8]>) -> VM {
        let inter = Interconnect::new(cart_rom);
        let cpu = Cpu::new();
        VM{
            inter: inter,
            cpu: cpu,
        }
    }

    pub fn run(&mut self) {
        self.cpu.run(&mut self.inter);
    }
}
