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
            "A: {:02X} X: {:02X} Y: {:02X} P: {:02X} SP: {:02X}",
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

        match *instr.opcode() {
            Opcode::Bit => {
                let mask = instr.imm(self.reg_pc, self.reg_x, self.reg_y);
                let result = self.reg_a & mask;

                self.reg_status.zero = result == 0;
                self.reg_status.overflow = (result & (1 << 6)) != 0;
                self.reg_status.negative = (result & (1 << 7)) != 0;
            }
            Opcode::Clc => {
                self.reg_status.carry = false;
            }
            Opcode::Jsr => {
                let pc = self.reg_pc;
                let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                self.push_word(interconnect, pc);
                self.reg_pc = addr;
            }
            Opcode::Sec => {
                self.reg_status.carry = true;
            }
            Opcode::Jmp => {
                let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                self.reg_pc = addr;
            },
            Opcode::Sta => {
                let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                interconnect.write_byte(addr, self.reg_a);
            }
            Opcode::Stx => {
                let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                interconnect.write_byte(addr, self.reg_x);
            }
            Opcode::Bcc => {
                if !self.reg_status.carry {
                    let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                    self.reg_pc += addr;
                }
            }
            Opcode::Ldx => {
                self.reg_x = instr.imm(self.reg_pc, self.reg_x, self.reg_y);
                self.reg_status.zero = self.reg_x == 0;
                self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            }
            Opcode::Lda => {
                self.reg_a = instr.imm(self.reg_pc, self.reg_x, self.reg_y);
                self.reg_status.zero = self.reg_a == 0;
                self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            }
            Opcode::Bcs => {
                if self.reg_status.carry {
                let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                    self.reg_pc += addr;
                }
            }
            Opcode::Bne => {
                if !self.reg_status.zero {
                let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                    self.reg_pc += addr;
                }
            }
            Opcode::Nop => {},
            Opcode::Beq => {
                if self.reg_status.zero {
                    let addr = instr.addr(self.reg_pc, self.reg_x, self.reg_y);
                    self.reg_pc += addr;
                }
            }
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
