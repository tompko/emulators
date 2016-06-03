use super::interconnect::Interconnect;
use super::instruction::Instruction;

pub struct Cpu {
    reg_pc: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu{
            reg_pc: 0xc000, // TODO - this should possibly start at the reset vector
        }
    }

    pub fn run(&mut self, interconnect: &mut Interconnect) {
        loop {
            let instr = self.read_instruction(interconnect);
            self.execute_instruction(interconnect, &instr);
        }
    }

    fn read_instruction(&self, interconnect: &mut Interconnect) -> Instruction {
        return Instruction::new(interconnect, self.reg_pc);
    }

    fn execute_instruction(&mut self, interconnect: &mut Interconnect, instr: &Instruction) {
    }
}
