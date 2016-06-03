use super::interconnect::Interconnect;
use super::num::FromPrimitive;

enum_from_primitive!{
#[derive(Debug)]
pub enum Opcode {
}
}

pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(interconnect: &Interconnect, pc: u16) -> Instruction {

        let opcode_byte = interconnect.read_byte(pc);
        let opcode = Opcode::from_u8(opcode_byte)
            .unwrap_or_else(|| panic!("Unrecognised opcode {:#x}", opcode_byte));
        return Instruction{
            opcode: opcode,
        }
    }

    pub fn opcode(&self) -> &Opcode {
        return &self.opcode;
    }
}
