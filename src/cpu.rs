use super::interconnect::Interconnect;
use super::instruction::{Instruction, Opcode};
use super::reg_status::RegStatus;

pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_pc: u16,
    reg_s: u8,
    reg_status: RegStatus,
}

impl Default for Cpu {
    fn default() -> Cpu {
        Cpu{
            reg_a: 0,
            reg_x: 0,
            reg_y: 0,
            reg_pc: 0xc000, // TODO - this should possibly start at the reset vector
            reg_s: 0,
            reg_status: 0.into(),
        }
    }
}

impl Cpu {
    pub fn run(&mut self, interconnect: &mut Interconnect) {
        loop {
            let instr = self.read_instruction(interconnect);
            self.execute_instruction(interconnect, &instr);
        }
    }

    fn read_instruction(&self, interconnect: &mut Interconnect) -> Instruction {
        Instruction::new(interconnect, self.reg_pc)
    }

    fn execute_instruction(&mut self, interconnect: &mut Interconnect, instr: &Instruction) {
        self.reg_pc += instr.length();

        match instr.opcode() {
            &Opcode::Jmp => {
                self.reg_pc = instr.addr();
            },
            &Opcode::Ldx => {
                self.reg_x = instr.imm();
                self.reg_status.zero = self.reg_x == 0;
                self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            }
        }
    }
}
