use std::fmt;
use super::interconnect::Interconnect;
use super::reg_status::RegStatus;

pub struct Cpu {
    reg_a: u8,
    reg_x: u8,
    reg_y: u8,
    reg_pc: u16,
    reg_s: u8,
    reg_status: RegStatus,

    instr_pc: u16,
    opcode: u8,
    time: u8,  // cycle of opcode we're executing next
    fetch: u8, // in the real 6502 this would sit on the data bus
    address: u16,
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

            instr_pc: 0,
            opcode: 0,
            time: 0,
            fetch: 0,
            address: 0,
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
            self.execute_cycle(interconnect);
        }
    }

    #[allow(unknown_lints)]
    #[allow(cyclomatic_complexity)]
    fn execute_cycle(&mut self, interconnect: &mut Interconnect) {
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x00 == 0x00 && self.time == 0 {
            // T0A
            unimplemented!();
        }
        if self.opcode & 0x84 == 0x84 && !self.opcode & 0x60 == 0x60 && g & 3 == 0 {
            // STY
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 3 {
            // T3INDYA
            unimplemented!();
        }
        if self.opcode & 0x18 == 0x18 && !self.opcode & 0x04 == 0x04 && g & 1 == 1 && self.time == 2 {
            // T2ABSY
            unimplemented!();
        }
        if self.opcode & 0xc0 == 0xc0 && !self.opcode & 0x30 == 0x30 && g & 3 == 0 && self.time == 0 {
            // T0CPYINY
            unimplemented!();
        }
        if self.opcode & 0x98 == 0x98 && !self.opcode & 0x64 == 0x64 && g & 3 == 0 && self.time == 0 {
            // T0TYAA
            unimplemented!();
        }
        if self.opcode & 0x88 == 0x88 && !self.opcode & 0x34 == 0x34 && g & 3 == 0 && self.time == 0 {
            // T0DEYINY
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xfc == 0xfc && g & 3 == 0 && self.time == 5 {
            // T5INT
            unimplemented!();
        }
        if self.opcode & 0x80 == 0x80 && !self.opcode & 0x40 == 0x40 && g & 2 == 2 {
            // LDXSDX
            unimplemented!();
        }
        if self.opcode & 0x14 == 0x14 && !self.opcode & 0x00 == 0x00 && self.time == 2 {
            // T2ANYX
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x1c == 0x1c && g & 1 == 1 && self.time == 2 {
            // T2XIND
            unimplemented!();
        }
        if self.opcode & 0x88 == 0x88 && !self.opcode & 0x74 == 0x74 && g & 2 == 2 && self.time == 0 {
            // T0TXAA
            unimplemented!();
        }
        if self.opcode & 0xc8 == 0xc8 && !self.opcode & 0x34 == 0x34 && g & 2 == 2 && self.time == 0 {
            // T0DEX
            unimplemented!();
        }
        if self.opcode & 0xe0 == 0xe0 && !self.opcode & 0x10 == 0x10 && g & 3 == 0 && self.time == 0 {
            // T0CPXINX
            unimplemented!();
        }
        if self.opcode & 0x98 == 0x98 && !self.opcode & 0x64 == 0x64 && g & 2 == 2 && self.time == 0 {
            // T0TXS
            unimplemented!();
        }
        if self.opcode & 0x80 == 0x80 && !self.opcode & 0x60 == 0x60 && g & 2 == 2 {
            // SDX
            unimplemented!();
        }
        if self.opcode & 0xa0 == 0xa0 && !self.opcode & 0x40 == 0x40 && g & 2 == 2 && self.time == 0 {
            // T0TALDTSX
            unimplemented!();
        }
        if self.opcode & 0xc8 == 0xc8 && !self.opcode & 0x34 == 0x34 && g & 2 == 2 && self.time == 1 {
            // T1DEX
            unimplemented!();
        }
        if self.opcode & 0xe8 == 0xe8 && !self.opcode & 0x14 == 0x14 && g & 3 == 0 && self.time == 1 {
            // T1INX
            unimplemented!();
        }
        if self.opcode & 0xb8 == 0xb8 && !self.opcode & 0x44 == 0x44 && g & 2 == 2 && self.time == 0 {
            // T0TSX
            unimplemented!();
        }
        if self.opcode & 0x88 == 0x88 && !self.opcode & 0x34 == 0x34 && g & 3 == 0 && self.time == 1 {
            // T1DEYINY
            unimplemented!();
        }
        if self.opcode & 0xa4 == 0xa4 && !self.opcode & 0x40 == 0x40 && g & 3 == 0 && self.time == 0 {
            // T0LDY1
            unimplemented!();
        }
        if self.opcode & 0xa0 == 0xa0 && !self.opcode & 0x50 == 0x50 && g & 3 == 0 && self.time == 0 {
            // T0LDY2TAY
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 && self.time == 2 {
            // CCC
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 && self.time == 0 {
            // T0JSR
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0xb4 == 0xb4 && g & 3 == 0 && self.time == 0 {
            // T0PSHASHP
            unimplemented!();
        }
        if self.opcode & 0x60 == 0x60 && !self.opcode & 0x9c == 0x9c && g & 3 == 0 && self.time == 4 {
            // T4RTS
            unimplemented!();
        }
        if self.opcode & 0x28 == 0x28 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 && self.time == 3 {
            // T3PLAPLPA
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0xbc == 0xbc && g & 3 == 0 && self.time == 5 {
            // T5RTI
            unimplemented!();
        }
        if self.opcode & 0x60 == 0x60 && !self.opcode & 0x80 == 0x80 && g & 2 == 2 {
            // RORRORA
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 && self.time == 2 {
            // T2JSR
            unimplemented!();
        }
        if self.opcode & 0x4c == 0x4c && !self.opcode & 0x90 == 0x90 && g & 3 == 0 {
            // JMPA
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x00 == 0x00 && self.time == 2 {
            // T2
            unimplemented!();
        }
        if self.opcode & 0x0c == 0x0c && !self.opcode & 0x10 == 0x10 && self.time == 2 {
            // T2EXT
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0x9c == 0x9c && g & 3 == 0 {
            // RTIRTS
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x1c == 0x1c && g & 1 == 1 && self.time == 4 {
            // T4XIND
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x08 == 0x08 && self.time == 2 {
            // T2NANYABS
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0xbc == 0xbc && g & 3 == 0 && self.time == 4 {
            // T4RTIA
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 && self.time == 4 {
            // T4JSRINT
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x90 == 0x90 && g & 3 == 0 && self.time == 3 {
            // NAME1:T3_RTI_RTS_JSR_JMP_INT_PULA_PUPL
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 3 {
            // T3INDYB
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x1c == 0x1c && g & 1 == 1 && self.time == 3 {
            // T3XIND
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 4 {
            // T4INDYA
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 2 {
            // T2INDY
            unimplemented!();
        }
        if self.opcode & 0x18 == 0x18 && !self.opcode & 0x00 == 0x00 && self.time == 3 {
            // T3ABSXYA
            unimplemented!();
        }
        if self.opcode & 0x28 == 0x28 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 {
            // PULAPULP
            unimplemented!();
        }
        if self.opcode & 0xe0 == 0xe0 && !self.opcode & 0x00 == 0x00 && g & 2 == 2 {
            // INC
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0xa0 == 0xa0 && g & 1 == 1 && self.time == 0 {
            // T0EOR
            unimplemented!();
        }
        if self.opcode & 0xc0 == 0xc0 && !self.opcode & 0x20 == 0x20 && g & 1 == 1 && self.time == 0 {
            // T0CMP
            unimplemented!();
        }
        if self.opcode & 0xc0 == 0xc0 && !self.opcode & 0x10 == 0x10 && g & 3 == 0 && self.time == 0 {
            // NAME2:T0_CPX_CPY_INX_INY
            unimplemented!();
        }
        if self.opcode & 0x60 == 0x60 && !self.opcode & 0x00 == 0x00 && g & 1 == 1 && self.time == 0 {
            // T0ADCSBC
            unimplemented!();
        }
        if self.opcode & 0xe0 == 0xe0 && !self.opcode & 0x00 == 0x00 && g & 1 == 1 && self.time == 0 {
            // T0SBC
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xc0 == 0xc0 && g & 2 == 2 {
            // ROLROLA
            unimplemented!();
        }
        if self.opcode & 0x4c == 0x4c && !self.opcode & 0x90 == 0x90 && g & 3 == 0 && self.time == 3 {
            // T3JMP
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xe0 == 0xe0 && g & 1 == 1 && self.time == 0 {
            // T0ORA
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xc0 == 0xc0 && g & 2 == 2 {
            // NAME8:ROL_ROLA_ASL_ASLA
            unimplemented!();
        }
        if self.opcode & 0x98 == 0x98 && !self.opcode & 0x64 == 0x64 && g & 3 == 0 && self.time == 0 {
            // T0TYAB
            unimplemented!();
        }
        if self.opcode & 0x88 == 0x88 && !self.opcode & 0x74 == 0x74 && g & 2 == 2 && self.time == 0 {
            // T0TXAB
            unimplemented!();
        }
        if self.opcode & 0x60 == 0x60 && !self.opcode & 0x00 == 0x00 && g & 1 == 1 && self.time == 1 {
            // T1ADCSBCA
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x80 == 0x80 && g & 1 == 1 && self.time == 1 {
            // NAME7:T1_AND_EOR_OR_ADC
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0x94 == 0x94 && g & 2 == 2 && self.time == 1 {
            // NAME4:T1_ASLA_ROLA_LSRA
            unimplemented!();
        }
        if self.opcode & 0x68 == 0x68 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 && self.time == 0 {
            // T0PULA
            unimplemented!();
        }
        if self.opcode & 0x18 == 0x18 && !self.opcode & 0x00 == 0x00 && self.time == 4 {
            // T4ABSXYA
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 5 {
            // T5INDY
            unimplemented!();
        }
        if self.opcode & 0xa0 == 0xa0 && !self.opcode & 0x40 == 0x40 && g & 1 == 1 && self.time == 0 {
            // T0LDA
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x00 == 0x00 && g & 1 == 1 && self.time == 0 {
            // T0G1
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xc0 == 0xc0 && g & 1 == 1 && self.time == 0 {
            // T0AND
            unimplemented!();
        }
        if self.opcode & 0x24 == 0x24 && !self.opcode & 0xd0 == 0xd0 && g & 3 == 0 && self.time == 0 {
            // T0BITA
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0x94 == 0x94 && g & 2 == 2 && self.time == 0 {
            // NAME6:T0_ASLA_ROLA_LSRA
            unimplemented!();
        }
        if self.opcode & 0xa8 == 0xa8 && !self.opcode & 0x54 == 0x54 && g & 2 == 2 && self.time == 0 {
            // T0TAX
            unimplemented!();
        }
        if self.opcode & 0xa8 == 0xa8 && !self.opcode & 0x54 == 0x54 && g & 3 == 0 && self.time == 0 {
            // T0TAY
            unimplemented!();
        }
        if self.opcode & 0x48 == 0x48 && !self.opcode & 0x94 == 0x94 && g & 2 == 2 && self.time == 0 {
            // T0LSRA
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0x80 == 0x80 && g & 2 == 2 {
            // LSRLSRA
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 && self.time == 5 {
            // T5JSRA
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 3 == 0 && self.time == 2 {
            // T2BR
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xfc == 0xfc && g & 3 == 0 && self.time == 2 {
            // T2INT
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 && self.time == 3 {
            // T3JSR
            unimplemented!();
        }
        if self.opcode & 0x04 == 0x04 && !self.opcode & 0x08 == 0x08 && self.time == 2 {
            // T2ANYZP
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 2 {
            // T2ANYIND
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x00 == 0x00 && self.time == 4 {
            // T4
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x00 == 0x00 && self.time == 3 {
            // T3
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xbc == 0xbc && g & 3 == 0 && self.time == 0 {
            // T0RTIINT
            unimplemented!();
        }
        if self.opcode & 0x4c == 0x4c && !self.opcode & 0x90 == 0x90 && g & 3 == 0 && self.time == 0 {
            // T0JMP
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 && self.time == 2 {
            // NAME3:T2_RTI_RTS_JSR_INT_PULA_PUPLP_PSHA_PSHP
            unimplemented!();
        }
        if self.opcode & 0x60 == 0x60 && !self.opcode & 0x9c == 0x9c && g & 3 == 0 && self.time == 5 {
            // T5RTS
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0x00 == 0x00 && self.time == 2 {
            // T2ANYABS
            unimplemented!();
        }
        if self.opcode & 0x80 == 0x80 && !self.opcode & 0x60 == 0x60 && g & 1 == 1 {
            // STA
            unimplemented!();
        }
        if self.opcode & 0x48 == 0x48 && !self.opcode & 0xb4 == 0xb4 && g & 3 == 0 && self.time == 2 {
            // T2PSHA
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 3 == 0 && self.time == 0 {
            // T0BR
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 {
            // PSHPULA
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x1c == 0x1c && g & 1 == 1 && self.time == 5 {
            // T5XIND
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0x00 == 0x00 && self.time == 3 {
            // T3ANYABS
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 4 {
            // T4INDYB
            unimplemented!();
        }
        if self.opcode & 0x18 == 0x18 && !self.opcode & 0x00 == 0x00 && self.time == 3 {
            // T3ABSXYB
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xbc == 0xbc && g & 3 == 0 {
            // RTIINT
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 {
            // JSR
            unimplemented!();
        }
        if self.opcode & 0x4c == 0x4c && !self.opcode & 0x90 == 0x90 && g & 3 == 0 {
            // JMPB
            unimplemented!();
        }
        if self.opcode & 0xc0 == 0xc0 && !self.opcode & 0x18 == 0x18 && g & 3 == 0 && self.time == 1 {
            // T1CPX2CY2
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0xd4 == 0xd4 && g & 2 == 2 && self.time == 1 {
            // T1ASLARLA
            unimplemented!();
        }
        if self.opcode & 0xcc == 0xcc && !self.opcode & 0x10 == 0x10 && g & 3 == 0 && self.time == 1 {
            // T1CPX1CY1
            unimplemented!();
        }
        if self.opcode & 0xc0 == 0xc0 && !self.opcode & 0x20 == 0x20 && g & 1 == 1 && self.time == 1 {
            // T1CMP
            unimplemented!();
        }
        if self.opcode & 0x60 == 0x60 && !self.opcode & 0x00 == 0x00 && g & 1 == 1 && self.time == 1 {
            // T1ADCSBCB
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xc0 == 0xc0 && g & 2 == 2 {
            // NAME5:ROL_ROLA_ASL_ASLA
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0x00 == 0x00 && g & 2 == 2 {
            // LSRRADCIC
            unimplemented!();
        }
        if self.opcode & 0x24 == 0x24 && !self.opcode & 0xd0 == 0xd0 && g & 3 == 0 && self.time == 1 {
            // T1BIT
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0xf4 == 0xf4 && g & 3 == 0 && self.time == 2 {
            // T2PSHP
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0xfc == 0xfc && g & 3 == 0 && self.time == 4 {
            // T4INT
            unimplemented!();
        }
        if self.opcode & 0x80 == 0x80 && !self.opcode & 0x60 == 0x60 {
            // STASTYSTX
            unimplemented!();
        }
        if self.opcode & 0x18 == 0x18 && !self.opcode & 0x00 == 0x00 && self.time == 4 {
            // T4ABSXYB
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x0c == 0x0c && g & 1 == 1 && self.time == 5 {
            // T5ANYIND
            unimplemented!();
        }
        if self.opcode & 0x04 == 0x04 && !self.opcode & 0x18 == 0x18 && self.time == 2 {
            // T2ZP
            unimplemented!();
        }
        if self.opcode & 0x0c == 0x0c && !self.opcode & 0x10 == 0x10 && self.time == 3 {
            // T3ABS
            unimplemented!();
        }
        if self.opcode & 0x14 == 0x14 && !self.opcode & 0x08 == 0x08 && self.time == 3 {
            // T3ZPX
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0xb4 == 0xb4 && g & 3 == 0 && self.time == 2 {
            // T2PSHASHP
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0x9c == 0x9c && g & 3 == 0 && self.time == 5 {
            // T5RTIRTS
            unimplemented!();
        }
        if self.opcode & 0x20 == 0x20 && !self.opcode & 0xdc == 0xdc && g & 3 == 0 && self.time == 5 {
            // T5JSRB
            unimplemented!();
        }
        if self.opcode & 0x4c == 0x4c && !self.opcode & 0x90 == 0x90 && g & 3 == 0 && self.time == 5 {
            // T4JMP
            unimplemented!();
        }
        if self.opcode & 0x4c == 0x4c && !self.opcode & 0xb0 == 0xb0 && g & 3 == 0 && self.time == 2 {
            // T2JMPABS
            unimplemented!();
        }
        if self.opcode & 0x28 == 0x28 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 && self.time == 3 {
            // T3PLAPLPB
            unimplemented!();
        }
        if self.opcode & 0x10 == 0x10 && !self.opcode & 0x0c == 0x0c && g & 3 == 0 && self.time == 3 {
            // T3BR
            unimplemented!();
        }
        if self.opcode & 0x24 == 0x24 && !self.opcode & 0xd0 == 0xd0 && g & 3 == 0 && self.time == 0 {
            // T0BITB
            unimplemented!();
        }
        if self.opcode & 0x40 == 0x40 && !self.opcode & 0xbc == 0xbc && g & 3 == 0 && self.time == 4 {
            // T4RTIB
            unimplemented!();
        }
        if self.opcode & 0x28 == 0x28 && !self.opcode & 0xd4 == 0xd4 && g & 3 == 0 && self.time == 0 {
            // T0PULP
            unimplemented!();
        }
        if self.opcode & 0x08 == 0x08 && !self.opcode & 0x94 == 0x94 && g & 3 == 0 {
            // PSHPULB
            unimplemented!();
        }
        if self.opcode & 0xb8 == 0xb8 && !self.opcode & 0x44 == 0x44 && g & 3 == 0 {
            // CLV
            unimplemented!();
        }
        if self.opcode & 0x18 == 0x18 && !self.opcode & 0xc4 == 0xc4 && g & 3 == 0 && self.time == 0 {
            // T0CLCSEC
            unimplemented!();
        }
        if self.opcode & 0x58 == 0x58 && !self.opcode & 0x84 == 0x84 && g & 3 == 0 && self.time == 0 {
            // T0CLISEI
            unimplemented!();
        }
        if self.opcode & 0xd8 == 0xd8 && !self.opcode & 0x04 == 0x04 && g & 3 == 0 && self.time == 0 {
            // T0CLDSED
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x80 == 0x80 {
            // NI7P
            unimplemented!();
        }
        if self.opcode & 0x00 == 0x00 && !self.opcode & 0x40 == 0x40 {
            // NI6P
            unimplemented!();
        }
        self.time += 1;
    }

    fn stack_push(&mut self, interconnect: &mut Interconnect, val: u8) {
        let addr = 0x0100 + self.reg_s as u16;
        interconnect.write_byte(addr, val);
        self.reg_s -= 1;
    }

    fn stack_peek(&self, interconnect: &Interconnect) -> u8 {
        let addr = 0x0100 + self.reg_s as u16;
        interconnect.read_byte(addr)
    }

    fn is_branch(&self, opcode: u8) -> bool {
        opcode & 16 == 16 && !opcode & 15 == 15
    }

    fn is_absolute(&self, opcode:u8) -> bool {
        opcode & 12 == 12 && !opcode & 16 == 16
    }

    fn is_absolute_y(&self, opcode: u8) -> bool {
        opcode & 25 == 25 && !opcode & 6 == 6
    }

    fn is_zero_page(&self, opcode: u8) -> bool {
        opcode & 4 == 4 && !opcode & 24 == 24
    }

    fn is_zero_indexed(&self, opcode: u8) -> bool {
        opcode & 20 == 20 && !opcode & 10 == 10
    }

    fn is_indexed_indirect(&self, opcode: u8) -> bool {
        opcode & 1 == 1 && !opcode & 30 == 30
    }

    fn is_indirect_indexed(&self, opcode: u8) -> bool {
        opcode & 17 == 17 && !opcode & 14 == 14
    }

    fn adc(&mut self, val: u8) {
        let acc = self.reg_a;
        let carry = if self.reg_status.carry { 1 } else { 0 };

        let (int, carry1) = val.overflowing_add(acc);
        let (fin, carry2) = int.overflowing_add(carry);

        self.reg_a = fin;

        self.reg_status.zero = self.reg_a == 0;
        self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
        self.reg_status.carry = carry1 || carry2;
        self.reg_status.overflow = !(acc ^ val) & (acc ^ fin) & 0x80 != 0;
    }

    fn and(&mut self, mask: u8) {
        self.reg_a &= mask;

        self.reg_status.zero = self.reg_a == 0;
        self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
    }

    fn asl(&mut self, val: u8) -> u8 {
        let res = val << 1;

        self.reg_status.carry = (val >> 7) != 0;
        self.reg_status.zero = res == 0;
        self.reg_status.negative = (res & (1 << 7)) != 0;

        res
    }

    fn bit(&mut self, mask: u8) {
        let value = self.reg_a & mask;

        self.reg_status.zero = value == 0;
        self.reg_status.overflow = (mask & (1 << 6)) != 0;
        self.reg_status.negative = (mask & (1 << 7)) != 0;
    }

    fn cmp(&mut self, val: u8) {
        let res = self.reg_a.wrapping_sub(val);

        self.reg_status.zero = self.reg_a == val;
        self.reg_status.carry = self.reg_a >= val;
        self.reg_status.negative = (res & (1 << 7)) != 0;
    }

    fn cpx(&mut self, val: u8) {
        let res = self.reg_x.wrapping_sub(val);

        self.reg_status.carry = self.reg_x >= val;
        self.reg_status.zero = self.reg_x == val;
        self.reg_status.negative = (res & (1 << 7)) != 0;
    }

    fn cpy(&mut self, val: u8) {
        let res = self.reg_y.wrapping_sub(val);

        self.reg_status.carry = self.reg_y >= val;
        self.reg_status.zero = self.reg_y == val;
        self.reg_status.negative = (res & (1 << 7)) != 0;
    }

    fn dec(&mut self, val: u8) -> u8 {
        let res = val.wrapping_sub(1);

        self.reg_status.zero = res == 0;
        self.reg_status.negative = (res & (1 << 7)) != 0;
        res
    }

    fn eor(&mut self, mask: u8) {
        self.reg_a ^= mask;

        self.reg_status.zero = self.reg_a == 0;
        self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
    }

    fn inc(&mut self, val: u8) -> u8 {
        let res = val.wrapping_add(1);

        self.reg_status.zero = res == 0;
        self.reg_status.negative = (res & (1 << 7)) != 0;
        res
    }

    fn lda(&mut self, val: u8) {
        self.reg_a = val;
        self.reg_status.zero = val == 0;
        self.reg_status.negative = (val & (1 << 7)) != 0;
    }

    fn ldy(&mut self, val: u8) {
        self.reg_y = val;
        self.reg_status.zero = val == 0;
        self.reg_status.negative = (val & (1 << 7)) != 0;
    }

    fn lsr(&mut self, val: u8) -> u8 {
        let res = val >> 1;

        self.reg_status.carry = val & 0x1 != 0;
        self.reg_status.zero = res == 0;
        self.reg_status.negative = false;

        res
    }

    fn ora(&mut self, mask: u8) {
        self.reg_a |= mask;

        self.reg_status.zero = self.reg_a == 0;
        self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
    }

    fn rol(&mut self, val: u8) -> u8 {
        let res = val << 1;
        let carry = if self.reg_status.carry { 1 } else { 0 };
        let res = res | carry;

        self.reg_status.carry = val & (1 << 7) != 0;
        self.reg_status.zero = res == 0;
        self.reg_status.negative = (res & (1 << 7)) != 0;
        res
    }

    fn ror(&mut self, val: u8) -> u8 {
        let res = val >> 1;
        let carry = if self.reg_status.carry { 1 } else { 0 };
        let res = res | (carry << 7);

        self.reg_status.carry = val & 0x1 != 0;
        self.reg_status.zero = res == 0;
        self.reg_status.negative = (res & (1 << 7)) != 0;
        res
    }

    fn take_branch(&self, opcode: u8) -> bool {
        match opcode {
            0x10 => !self.reg_status.negative,
            0x30 => self.reg_status.negative,
            0x50 => !self.reg_status.overflow,
            0x70 => self.reg_status.overflow,
            0x90 => !self.reg_status.carry,
            0xb0 => self.reg_status.carry,
            0xd0 => !self.reg_status.zero,
            0xf0 => self.reg_status.zero,
            _ => panic!("Unrecognised branch instruction {:02X}", opcode),
        }
    }

    fn opcode_name(&self, opcode: u8) -> &'static str {
        match opcode {
            0x01 | 0x0d => "ORA",
            0x10 => "BPL",
            0x11 => "ORA",
            0x21 => "AND",
            0x30 => "BMI",
            0x31 => "AND",
            0x41 => "EOR",
            0x50 => "BVC",
            0x51 => "EOR",
            0x61 | 0x69 | 0x71 => "ADC",
            0x70 => "BVS",
            0x81 => "STA",
            0x90 => "BCC",
            0x91 => "STA",
            0xa1 => "LDA",
            0xb0 => "BCS",
            0xb1 => "LDA",
            0xc1 => "CMP",
            0xd0 => "BNE",
            0xd1 => "CMP",
            0xe1 | 0xe9 => "SBC",
            0xf0 => "BEQ",
            0xf1 => "SBC",
            _ => panic!("Unrecognised instruction {:02X}", opcode),
        }
    }
}
