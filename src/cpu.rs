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
            // T1 - All instructions
            self.instr_pc = self.reg_pc;
            self.opcode = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return
        }

        if self.opcode == 0x4c && self.time == 2 {
            // T2 - JMP
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x4c && self.time == 3 {
            // T3 - JMP
            let pch = interconnect.read_byte(self.reg_pc);
            self.reg_pc = ((pch as u16) << 8) | (self.fetch as u16);
            println!("{:04X}  {:02X} {:02X} {:02X}   JMP ${:04X}     {:?}", self.instr_pc, self.opcode, self.fetch, pch, self.reg_pc, self);
            self.time = 0;
            return;
        }

        // LDX
        if self.opcode == 0xa2 && self.time == 2 {
            // T2 - LDX Immediate
            let value = interconnect.read_byte(self.reg_pc);
            self.reg_x = value;
            self.reg_pc += 1;
            self.reg_status.zero = value == 0;
            self.reg_status.negative = (value & (1 << 7)) != 0;

            println!("{:04X}  {:02X} {:02X}      LDX #${:02X}      {:?}", self.instr_pc, self.opcode, self.reg_x, self.reg_x, self);
            self.time = 0;
            return;
        }

        // LDA
        if self.opcode == 0xa9 && self.time == 2 {
            let value = interconnect.read_byte(self.reg_pc);
            self.reg_a = value;
            self.reg_pc += 1;
            self.reg_status.zero = value == 0;
            self.reg_status.negative = (value & (1 << 7)) != 0;

            println!("{:04X}  {:02X} {:02X}      LDA #${:02X}      {:?}", self.instr_pc, self.opcode, self.reg_a, self.reg_a, self);
            self.time = 0;
            return;
        }

        if (self.opcode == 0x86 || self.opcode == 0x85 || self.opcode == 0x24) && self.time == 2 {
            // TODO - All Zero Page instructions should share this
            // T2 - STX/STA/BIT Zero Page
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x86 && self.time == 3 {
            interconnect.write_byte(self.fetch as u16, self.reg_x);
            println!("{:04X}  {:02X} {:02X}      STX ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_x, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x85 && self.time == 3 {
            interconnect.write_byte(self.fetch as u16, self.reg_a);
            println!("{:04X}  {:02X} {:02X}      STA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_a, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x24 && self.time == 3 {
            let mask = interconnect.read_byte(self.fetch as u16);
            let value = self.reg_a & mask;

            self.reg_status.zero = value == 0;
            self.reg_status.overflow = (mask & (1 << 6)) != 0;
            self.reg_status.negative = (mask & (1 << 7)) != 0;

            println!("{:04X}  {:02X} {:02X}      BIT ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, mask, self);
            self.time = 0;
            return;
        }

        // JSR
        if self.opcode == 0x20 && self.time == 2{
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x20 && self.time == 3{
            // internal operation (predecrement S?)
            return;
        }
        if self.opcode == 0x20 && self.time == 4{
            let byte = (self.reg_pc >> 8) as u8;
            self.push_byte(interconnect, byte);
            return;
        }
        if self.opcode == 0x20 && self.time == 5{
            let byte = self.reg_pc as u8;
            self.push_byte(interconnect, byte);
            return;
        }
        if self.opcode == 0x20 && self.time == 6{
            let pch = interconnect.read_byte(self.reg_pc);
            self.reg_pc = ((pch as u16) << 8) | (self.fetch as u16);
            println!("{:04X}  {:02X} {:02X} {:02X}   JSR ${:04X}    {:?}", self.instr_pc, self.opcode, self.fetch, pch, self.reg_pc, self);
            self.time = 0;
            return;
        }

        // NOP
        if self.opcode == 0xea && self.time == 2 {
            println!("{:04X}  {:02X}         NOP          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // SEC
        if self.opcode == 0x38 && self.time == 2 {
            self.reg_status.carry = true;
            println!("{:04X}  {:02X}         SEC          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // CLC
        if self.opcode == 0x18 && self.time == 2 {
            self.reg_status.carry = false;
            println!("{:04X}  {:02X}         SEC          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }

        // Relative branch - based on reg_status values
        if self.is_branch(self.opcode) && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.is_branch(self.opcode) && self.time == 3 {
            if self.take_branch(self.opcode) {
                // TODO - this operation should take 2 cycles
                // we should increment the low byte of pc in this cycle
                // and increment the high bytes in the next if necessary
                self.reg_pc += self.fetch as u16;
            } else {
            println!("{:04X}  {:02X} {:02X}      {} ${:04X}    {:?}", self.instr_pc, self.opcode, self.fetch, self.opcode_name(self.opcode), self.reg_pc + self.fetch as u16, self);
            self.time = 0;
            }
            return;
        }
        if self.is_branch(self.opcode) && self.time == 4 {
            // TODO - we should increment the high order byte of pc here if the
            // low order addition overflowed, and we should run for 5 cycles if
            // that happens
            println!("{:04X}  {:02X} {:02X}      {} ${:04X}    {:?}", self.instr_pc, self.opcode, self.fetch, self.opcode_name(self.opcode), self.reg_pc, self);
            self.time = 0;
            return;
        }
        if self.is_branch(self.opcode) && self.time == 5 {
        }

        panic!("Unmatched opcode/time pair {:x}/{}", self.opcode, self.time);
    }

    fn push_byte(&mut self, interconnect: &mut Interconnect, val: u8) {
        interconnect.write_byte(self.reg_s as u16, val);
        self.reg_s -= 1;
    }

    fn is_branch(&self, opcode: u8) -> bool {
        opcode & 16 == 16 && !opcode & 15 == 15
    }
    fn take_branch(&self, opcode: u8) -> bool {
        match opcode {
            0x10 => !self.reg_status.negative,
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
            0x10 => "BPL",
            0x50 => "BVC",
            0x70 => "BVS",
            0x90 => "BCC",
            0xb0 => "BCS",
            0xd0 => "BNE",
            0xf0 => "BEQ",
            _ => panic!("Unrecognised instruction {:02X}", opcode),
        }
    }
}
