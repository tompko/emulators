use std::fmt;
use super::interconnect::Interconnect;
use super::num::FromPrimitive;

enum_from_primitive!{
#[derive(Debug)]
pub enum Opcode {
    Jsr = 0x20,
    Sec = 0x38,
    Jmp = 0x4c,
    Stx = 0x86,
    Ldx = 0xa2,
    Bcs = 0xb0,
    Nop = 0xea,
}
}

pub struct Instruction {
    opcode: Opcode,
    addr: u16,
    imm: u8,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr_lo = self.addr as u8;
        let addr_hi = (self.addr >> 8) as u8;

        match self.opcode() {
            &Opcode::Jsr => write!(f, "20 {:02X} {:02X}  JSR ${:02X}  ", addr_lo, addr_hi, self.addr),
            &Opcode::Sec => write!(f, "38        SEC        "),
            &Opcode::Jmp => write!(f, "4C {:02X} {:02X}  JMP ${:02X}  ", addr_lo, addr_hi, self.addr),
            &Opcode::Stx => write!(f, "86 {:02X}     STX ${:02X} = X", addr_lo, self.addr),
            &Opcode::Ldx => write!(f, "A2 {:02X}     LDX #${:02X}   ", self.imm, self.imm),
            &Opcode::Bcs => write!(f, "B0        BCS {:02X}  ", self.addr),
            &Opcode::Nop => write!(f, "EA        NOP        "),
        }
    }
}

impl Instruction {
    pub fn new(interconnect: &Interconnect, pc: u16) -> Instruction {
        let opcode_byte = interconnect.read_byte(pc);
        let opcode = Opcode::from_u8(opcode_byte)
            .unwrap_or_else(|| panic!("Unrecognised opcode {:#x}({:x})", opcode_byte, pc));

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
            &Opcode::Jsr | &Opcode::Jmp => 3,
            &Opcode::Stx | &Opcode::Ldx | &Opcode::Bcs => 2,
            &Opcode::Nop | &Opcode::Sec => 1,
        }
    }

    fn read_operands(&mut self, interconnect: &Interconnect, pc: u16) {
        match self.opcode() {
            &Opcode::Jsr => self.addr = interconnect.read_word(pc),
            &Opcode::Jmp => self.addr = interconnect.read_word(pc),
            &Opcode::Stx => self.addr = interconnect.read_byte(pc) as u16,
            &Opcode::Ldx => self.imm = interconnect.read_byte(pc),
            &Opcode::Bcs => self.addr = interconnect.read_byte(pc) as u16,
            _ => {},
        }
    }
}
