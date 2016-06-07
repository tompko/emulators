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

    fn execute_cycle(&mut self, interconnect: &mut Interconnect) {
        self.time += 1;
        if self.time == 1 {
            self.instr_pc = self.reg_pc;
            self.opcode = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return
        }

        if self.opcode == 0x4c && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x4c && self.time == 3 {
            let pch = interconnect.read_byte(self.reg_pc);
            self.reg_pc = ((pch as u16) << 8) | (self.fetch as u16);
            println!("{:04X}  {:02X} {:02X} {:02X}  JMP ${:04X}", self.instr_pc, self.opcode, self.fetch, pch, self.reg_pc);
            self.time = 0;
            return;
        }

        panic!("Unmatched opcode/time pair {:x}/{}", self.opcode, self.time);
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
