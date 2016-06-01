use super::interconnect::Interconnect;

pub struct Cpu {
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu{
        }
    }

    pub fn run(&mut self, interconnect: &mut Interconnect) {
        unimplemented!();
    }
}
