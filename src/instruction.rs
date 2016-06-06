use std::fmt;
use super::interconnect::Interconnect;
use super::num::FromPrimitive;

#[derive(Debug)]
pub enum Opcode {
    Clc,
    Jsr,
    Bit,
    Sec,
    Jmp,
    Sta,
    Stx,
    Bcc,
    Ldx,
    Lda,
    Bcs,
    Bne,
    Nop,
    Beq,
}

#[derive(Debug)]
pub enum AddressMode {
    Implicit,
    Accumulator,
    Immediate(u8),
    ZeroPage(u8),
    ZeroPageX(u8),
    ZeroPageY(u8),
    Relative(u8),
    Absolute(u16),
    AbsoluteX(u16),
    AbsoluteY(u16),
    Indirect(u8),
    IndirectX(u8),
    IndirectY(u8),
}

pub struct Instruction {
    opcode: Opcode,
    opcode_byte: u8,
    address_mode: AddressMode,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.address_mode {
            AddressMode:: Absolute(n) => write!(f, "{:X} {:X} {:X} {:?} {:X}", self.opcode_byte, (n >> 8) as u8, n as u8, self.opcode, n),
            _ => panic!("Unrecognised addressing mode {:?}", self.address_mode),
        }
    }
}

impl Instruction {
    pub fn new(interconnect: &Interconnect, pc: u16) -> Instruction {
        let opcode_byte = interconnect.read_byte(pc);

        let mut instr = Self::from_u8(opcode_byte);
        instr.read_operands(interconnect, pc + 1);
        instr
    }

    pub fn opcode(&self) -> &Opcode {
        &self.opcode
    }

    pub fn addr(&self, pc: u16, x: u8, y: u8) -> u16 {
        match self.address_mode {
            AddressMode::Absolute(n) => n,
            _ => panic!("Unrecognised address mode")
        }
    }

    pub fn imm(&self, pc: u16, x: u8, y: u8) -> u8 {
        match self.address_mode {
            _ => panic!("Unrecognised address mode")
        }
    }

    pub fn length(&self) -> u16 {
        match self.address_mode {
            AddressMode::Absolute(_) => 3,
            _ => panic!("Unrecognised address mode")
        }
    }

    fn from_u8(opcode_byte: u8) -> Instruction {
        let mut opcode = Opcode::Nop;
        let mut address_mode = AddressMode::Implicit;
        match opcode_byte {
            0x4C => {opcode = Opcode::Jmp; address_mode = AddressMode::Absolute(0)},
            _ => panic!("Unrecognised opcode {:X}", opcode_byte),
        }

        Instruction{
            opcode: opcode,
            opcode_byte: opcode_byte,
            address_mode: address_mode,
        }
    }

    fn read_operands(&mut self, interconnect: &Interconnect, pc: u16) {
        let address_mode = match self.address_mode {
            AddressMode::Absolute(_) => AddressMode::Absolute(interconnect.read_word(pc)),
            _ => panic!("Unrecognised address mode {:?}", self.address_mode),
        };

        self.address_mode = address_mode
    }
}
