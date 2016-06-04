use std::fmt;
use super::interconnect::Interconnect;
use super::num::FromPrimitive;

enum_from_primitive!{
#[derive(Debug)]
pub enum Opcode {
    Jmp = 0x4c,
    Ldx = 0xa2,
}
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    addr: u16,
    imm: u8,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.opcode() {
            &Opcode::Jmp => write!(f, "JMP ${:X}", self.addr),
            &Opcode::Ldx => write!(f, "LDX #${:X}", self.imm),
        }
    }
}

impl Instruction {
    pub fn new(interconnect: &Interconnect, pc: u16) -> Instruction {

        let opcode_byte = interconnect.read_byte(pc);
        let opcode = Opcode::from_u8(opcode_byte)
            .unwrap_or_else(|| panic!("Unrecognised opcode {:#x}", opcode_byte));

        let mut instr = Instruction{
            opcode: opcode,
            addr: 0,
            imm: 0,
        };
        instr.read_operands(interconnect, pc + 1);
        instr
    }

    pub fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn addr(&self) -> u16 {
        self.addr
    }

    pub fn imm(&self) -> u8 {
        self.imm
    }

    pub fn length(&self) -> u16 {
        match self.opcode() {
            &Opcode::Jmp => 3,
            &Opcode::Ldx => 2,
        }
    }

    fn read_operands(&mut self, interconnect: &Interconnect, pc: u16) {
        match self.opcode() {
            &Opcode::Jmp => self.addr = interconnect.read_word(pc),
            &Opcode::Ldx => self.imm = interconnect.read_byte(pc),
        }
    }
}
