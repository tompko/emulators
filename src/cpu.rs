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
        // LDY
        if self.opcode == 0xa0 && self.time == 2 {
            let value = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.reg_y = value;
            self.reg_status.zero = value == 0;
            self.reg_status.negative = (value & (1 << 7)) != 0;

            println!("{:04X}  {:02X} {:02X}      LDY #${:02X}      {:?}", self.instr_pc, self.opcode, self.reg_y, self.reg_y, self);
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

        // PHP
        if self.opcode == 0x08 && self.time == 2 {
            // read the next instruction and discard it
            return;
        }
        if self.opcode == 0x08 && self.time == 3 {
            let mut reg_status = self.reg_status.clone();
            // PHP  always pushes the break_command flag as 1
            reg_status.break_command = true;
            let val: u8 = reg_status.into();
            self.stack_push(interconnect, val);
            println!("{:04X}  {:02X}         PHP           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // PHA
        if self.opcode == 0x48 && self.time == 2 {
            // read the next instruction and discard it
            return;
        }
        if self.opcode == 0x48 && self.time == 3 {
            let val = self.reg_a;
            self.stack_push(interconnect, val);
            println!("{:04X}  {:02X}         PHA           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }

        // PLA
        if self.opcode == 0x68 && self.time == 2 {
            // read the next instruction and discard
            return;
        }
        if self.opcode == 0x68 && self.time == 3 {
            self.reg_s += 1;
            return;
        }
        if self.opcode == 0x68 && self.time == 4 {
            let val = self.stack_peek(interconnect);
            self.reg_a = val;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         PLA           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // PLP
        if self.opcode == 0x28 && self.time == 2 {
            // read the next instruction and discard
            return;
        }
        if self.opcode == 0x28 && self.time == 3 {
            self.reg_s += 1;
            return;
        }
        if self.opcode == 0x28 && self.time == 4 {
            let val = self.stack_peek(interconnect);
            // PLP never sets the break_command flag
            let val = val & 0xef;
            self.reg_status = val.into();
            println!("{:04X}  {:02X}         PLP           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }

        if (self.opcode == 0x84 || self.opcode == 0x86 || self.opcode == 0x85 || self.opcode == 0x24) && self.time == 2 {
            // TODO - All Zero Page instructions should share this
            // T2 - STY/STX/STA/BIT Zero Page
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x84 && self.time == 3 {
            interconnect.write_byte(self.fetch as u16, self.reg_y);
            println!("{:04X}  {:02X} {:02X}      STY ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_y, self);
            self.time = 0;
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

        // Absolute addressing
        if self.is_absolute(self.opcode) && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.is_absolute(self.opcode) && self.time == 3 {
            let addr_hi = interconnect.read_byte(self.reg_pc) as u16;
            self.reg_pc += 1;
            self.address = (addr_hi << 8) | self.fetch as u16;
            return
        }
        // STX Absolute
        if self.opcode == 0x8e && self.time == 4 {
            interconnect.write_byte(self.address, self.reg_x);
            println!("{:04X}  {:02X} {:02X} {:02X}   STX ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_x, self);
            self.time = 0;
            return;
        }
        // LDX Absolute
        if self.opcode == 0xae && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.reg_x = val;
            self.reg_status.zero = val == 0;
            self.reg_status.negative = (val & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X} {:02X}   LDX ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_x, self);
            self.time = 0;
            return;
        }
        // LDA Absolute
        if self.opcode == 0xad && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.reg_a = val;
            self.reg_status.zero = val == 0;
            self.reg_status.negative = (val & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X} {:02X}   LDA ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
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
            self.stack_push(interconnect, byte);
            return;
        }
        if self.opcode == 0x20 && self.time == 5{
            let byte = self.reg_pc as u8;
            self.stack_push(interconnect, byte);
            return;
        }
        if self.opcode == 0x20 && self.time == 6{
            let pch = interconnect.read_byte(self.reg_pc);
            self.reg_pc = ((pch as u16) << 8) | (self.fetch as u16);
            println!("{:04X}  {:02X} {:02X} {:02X}   JSR ${:04X}    {:?}", self.instr_pc, self.opcode, self.fetch, pch, self.reg_pc, self);
            self.time = 0;
            return;
        }

        // RTS
        if self.opcode == 0x60 && self.time == 2 {
            // Read next instruction and discard
            return;
        }
        if self.opcode == 0x60 && self.time == 3 {
            self.reg_s += 1;
            return;
        }
        if self.opcode == 0x60 && self.time == 4 {
            let pcl = self.stack_peek(interconnect) as u16;
            self.reg_s += 1;
            self.reg_pc = (self.reg_pc & 0xff00) | pcl;
            return;
        }
        if self.opcode == 0x60 && self.time == 5 {
            let pch = self.stack_peek(interconnect) as u16;
            self.reg_pc = (pch << 8) | (self.reg_pc & 0xff);
            return;
        }
        if self.opcode == 0x60 && self.time == 6 {
            self.reg_pc += 1;
            println!("{:04X}  {:02X}         RTS          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }

        // RTS
        if self.opcode == 0x40 && self.time == 2 {
            // read the next instruction and discard it
            return;
        }
        if self.opcode == 0x40 && self.time == 3 {
            self.reg_s += 1;
            return;
        }
        if self.opcode == 0x40 && self.time == 4 {
            let val = self.stack_peek(interconnect);
            self.reg_s += 1;
            self.reg_status = val.into();
            return;
        }
        if self.opcode == 0x40 && self.time == 5 {
            let pcl = self.stack_peek(interconnect) as u16;
            self.reg_pc = (self.reg_pc & 0xff00) | pcl;
            self.reg_s += 1;
            return;
        }
        if self.opcode == 0x40 && self.time == 6 {
            let pch = self.stack_peek(interconnect) as u16;
            self.reg_pc = (pch << 8) | (self.reg_pc & 0x00ff);
            println!("{:04X}  {:02X}         RTI          {:?}", self.instr_pc, self.opcode, self);
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
            println!("{:04X}  {:02X}         CLC          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // CLV
        if self.opcode == 0xb8 && self.time == 2 {
            self.reg_status.overflow = false;
            println!("{:04X}  {:02X}         CLV          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // SEI
        if self.opcode == 0x78 && self.time == 2 {
            self.reg_status.interrupt_disable = true;
            println!("{:04X}  {:02X}         SEI          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // CLI
        if self.opcode == 0x58 && self.time == 2 {
            self.reg_status.interrupt_disable = false;
            println!("{:04X}  {:02X}         CLI          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // SED
        if self.opcode == 0xf8 && self.time == 2 {
            // TODO - I think this is a NOP on the NES which always has decimal disabled
            self.reg_status.decimal = true;
            println!("{:04X}  {:02X}         SED          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // CLD
        if self.opcode == 0xD8 && self.time == 2 {
            // TODO - I think this is a NOP on the NES which always has decimal disabled
            self.reg_status.decimal = false;
            println!("{:04X}  {:02X}         CLD          {:?}", self.instr_pc, self.opcode, self);
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

        // INY
        if self.opcode == 0xc8 && self.time == 2 {
            self.reg_y = self.reg_y.wrapping_add(1);
            self.reg_status.zero = self.reg_y == 0;
            self.reg_status.negative = (self.reg_y & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         INY           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // INX
        if self.opcode == 0xe8 && self.time == 2 {
            self.reg_x = self.reg_x.wrapping_add(1);
            self.reg_status.zero = self.reg_x == 0;
            self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         INX           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // DEY
        if self.opcode == 0x88 && self.time == 2 {
            self.reg_y = self.reg_y.wrapping_sub(1);
            self.reg_status.zero = self.reg_y == 0;
            self.reg_status.negative = (self.reg_y & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         DEY           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // DEX
        if self.opcode == 0xca && self.time == 2 {
            self.reg_x = self.reg_x.wrapping_sub(1);
            self.reg_status.zero = self.reg_x == 0;
            self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         DEX           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // TAY
        if self.opcode == 0xa8 && self.time == 2 {
            self.reg_y = self.reg_a;
            self.reg_status.zero = self.reg_y == 0;
            self.reg_status.negative = (self.reg_y & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         TAY           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // TAX
        if self.opcode == 0xaa && self.time == 2 {
            self.reg_x = self.reg_a;
            self.reg_status.zero = self.reg_x == 0;
            self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         TAX           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // TYA
        if self.opcode == 0x98 && self.time == 2 {
            self.reg_a = self.reg_y;
            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         TYA           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // TXA
        if self.opcode == 0x8a && self.time == 2 {
            self.reg_a = self.reg_x;
            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         TXA           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // TSX
        if self.opcode == 0xba && self.time == 2 {
            self.reg_x = self.reg_s;
            self.reg_status.zero = self.reg_x == 0;
            self.reg_status.negative = (self.reg_x & (1 << 7)) != 0;
            println!("{:04X}  {:02X}         TSX           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // TXS
        if self.opcode == 0x9a && self.time == 2 {
            self.reg_s = self.reg_x;
            println!("{:04X}  {:02X}         TXS           {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }

        // ORA
        if self.opcode == 0x09 && self.time == 2 {
            let mask = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.reg_a |= mask;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      ORA #${:02X}      {:?}", self.instr_pc, self.opcode, mask, mask, self);
            self.time = 0;
            return;
        }
        // AND
        if self.opcode == 0x29 && self.time == 2 {
            let mask = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.reg_a &= mask;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      AND #${:02X}      {:?}", self.instr_pc, self.opcode, mask, mask, self);
            self.time = 0;
            return;
        }
        // EOR
        if self.opcode == 0x49 && self.time == 2 {
            let mask = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.reg_a ^= mask;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      EOR #${:02X}      {:?}", self.instr_pc, self.opcode, mask, mask, self);
            self.time = 0;
            return;
        }

        if (self.opcode == 0x69 || self.opcode == 0xe9) && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
        }
        // SBC
        if self.opcode == 0xe9 && self.time == 2 {
            self.fetch = !self.fetch;
        }
        // ADC
        if (self.opcode == 0x69 || self.opcode == 0xe9) && self.time == 2 {
            let val = self.fetch;
            let acc = self.reg_a;
            let carry = if self.reg_status.carry { 1 } else { 0 };
            self.reg_pc += 1;

            let (int, carry1) = val.overflowing_add(acc);
            let (fin, carry2) = int.overflowing_add(carry);

            self.reg_a = fin;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            self.reg_status.carry = carry1 || carry2;
            self.reg_status.overflow = !(acc ^ val) & (acc ^ fin) & 0x80 != 0;
            println!("{:04X}  {:02X} {:02X}      {} #${:02X}      {:?}", self.instr_pc, self.opcode, val, self.opcode_name(self.opcode), val, self);
            self.time = 0;
            return;
        }

        // CPY
        if self.opcode == 0xc0 && self.time == 2 {
            let cmp = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            let res = self.reg_y.wrapping_sub(cmp);

            self.reg_status.carry = self.reg_y >= cmp;
            self.reg_status.zero = self.reg_y == cmp;
            self.reg_status.negative = (res & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      CPY #${:02X}      {:?}", self.instr_pc, self.opcode, cmp, cmp, self);
            self.time = 0;
            return;
        }
        // CMP
        if self.opcode == 0xc9 && self.time == 2 {
            let cmp = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            let res = self.reg_a.wrapping_sub(cmp);

            self.reg_status.carry = self.reg_a >= cmp;
            self.reg_status.zero = self.reg_a == cmp;
            self.reg_status.negative = (res & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      CMP #${:02X}      {:?}", self.instr_pc, self.opcode, cmp, cmp, self);
            self.time = 0;
            return;
        }
        // CPX
        if self.opcode == 0xe0 && self.time == 2 {
            let cmp = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            let res = self.reg_x.wrapping_sub(cmp);

            self.reg_status.carry = self.reg_x >= cmp;
            self.reg_status.zero = self.reg_x == cmp;
            self.reg_status.negative = (res & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      CPX #${:02X}      {:?}", self.instr_pc, self.opcode, cmp, cmp, self);
            self.time = 0;
            return;
        }

        panic!("Unmatched opcode/time pair {:x}/{}", self.opcode, self.time);
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
            0x10 => "BPL",
            0x30 => "BMI",
            0x50 => "BVC",
            0x69 => "ADC",
            0x70 => "BVS",
            0x90 => "BCC",
            0xb0 => "BCS",
            0xd0 => "BNE",
            0xe9 => "SBC",
            0xf0 => "BEQ",
            _ => panic!("Unrecognised instruction {:02X}", opcode),
        }
    }
}
