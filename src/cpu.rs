use std::fmt;
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
            reg_s: 0xfd, // TODO - this should start at ff, probably wrong because of the reset
            reg_status: 0x24.into(),
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rs: u8 = self.reg_status.clone().into();
        write!(
            f,
            "A: {:X} X: {:X} Y: {:X} P: {:X} SP: {:X}",
            self.reg_a,
            self.reg_x,
            self.reg_y,
            rs,
            self.reg_s)
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
        println!("{:X}  {:?} {:?}", self.reg_pc, instr, self);
        self.reg_pc += instr.length();

        match instr.opcode() {
            &Opcode::Jsr => {
                let pc = self.reg_pc;
                self.push_word(interconnect, pc);
                self.reg_pc = instr.addr();
            }
            &Opcode::Sec => {
                self.reg_status.carry = true;
            }
            &Opcode::Jmp => {
                self.reg_pc = instr.addr();
            },
            &Opcode::Stx => {
                interconnect.write_byte(instr.addr(), self.reg_x);
            }
            &Opcode::Ldx => {
                self.reg_x = instr.imm();
                self.reg_status.zero = self.reg_x == 0;
                self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            }
            &Opcode::Bcs => {
                if self.reg_status.carry {
                    self.reg_pc += instr.addr();
                }
            }
            &Opcode::Nop => {},
        }
    }

    fn push_word(&mut self, interconnect: &mut Interconnect, val: u16) {
        self.push_byte(interconnect, (val >> 8) as u8);
        self.push_byte(interconnect, val as u8);
    }

    fn push_byte(&mut self, interconnect: &mut Interconnect, val: u8) {
        interconnect.write_byte(self.reg_s as u16, val);
        self.reg_s -= 1;
    }
}
