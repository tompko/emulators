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
        self.time += 1;
        if self.time == 1 {
            // T1 - All instructions
            self.instr_pc = self.reg_pc;
            self.opcode = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return
        }

        // NOP
        if self.opcode == 0xea && self.time == 2 {
            println!("{:04X}  {:02X}         NOP          {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }

        // JMP absolute
        if self.opcode == 0x4c && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x4c && self.time == 3 {
            let pch = interconnect.read_byte(self.reg_pc);
            self.reg_pc = ((pch as u16) << 8) | (self.fetch as u16);
            println!("{:04X}  {:02X} {:02X} {:02X}   JMP ${:04X}     {:?}", self.instr_pc, self.opcode, self.fetch, pch, self.reg_pc, self);
            self.time = 0;
            return;
        }
        // JMP absolute indirect
        if self.opcode == 0x6c && self.time == 2 {
            self.address = interconnect.read_byte(self.reg_pc) as u16;
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x6c && self.time == 3 {
            let addrhi = interconnect.read_byte(self.reg_pc) as u16;
            self.reg_pc += 1;
            self.address |= addrhi << 8;
            return
        }
        if self.opcode == 0x6c && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);

            // A quirk in the 6502 means the next fetch always takes place on 
            // the same page
            let addrlo = (self.address & 0xff) as u8;
            let addrhi = self.address >> 8;
            let addrlo = addrlo.wrapping_add(1);
            self.address = (addrhi << 8) | (addrlo as u16);

            return;
        }
        if self.opcode == 0x6c && self.time == 5 {
            let addrhi = interconnect.read_byte(self.address) as u16;
            self.reg_pc = (addrhi << 8) | (self.fetch as u16);
            println!("{:04X}  {:02X} {:02X} {:02X}   JMP (${:04X}) = {:04X}     {:?}", self.instr_pc, self.opcode, self.address & 0xff, self.address >> 8, self.address, self.reg_pc, self);
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
            self.ldy(value);

            println!("{:04X}  {:02X} {:02X}      LDY #${:02X}      {:?}", self.instr_pc, self.opcode, self.reg_y, self.reg_y, self);
            self.time = 0;
            return;
        }
        // LDA
        if self.opcode == 0xa9 && self.time == 2 {
            let value = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.lda(value);

            println!("{:04X}  {:02X} {:02X}      LDA #${:02X}      {:?}", self.instr_pc, self.opcode, self.reg_a, self.reg_a, self);
            self.time = 0;
            return;
        }
        // ASL - Accumulator
        if self.opcode == 0x0a && self.time == 2 {
            let acc = self.reg_a;
            let val = self.asl(acc);
            self.reg_a = val;

            println!("{:04X}  {:02X}         ASL A       {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // LSR - Accumulator
        if self.opcode == 0x4a && self.time == 2 {
            let acc = self.reg_a;
            let val = self.lsr(acc);
            self.reg_a = val;

            println!("{:04X}  {:02X}         LSR A       {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // ROR - Accumulator
        if self.opcode == 0x6a && self.time == 2 {
            let acc = self.reg_a;
            let val = self.ror(acc);
            self.reg_a = val;

            println!("{:04X}  {:02X}         ROR A       {:?}", self.instr_pc, self.opcode, self);
            self.time = 0;
            return;
        }
        // ROL - Accumulator
        if self.opcode == 0x2a && self.time == 2 {
            let acc = self.reg_a;
            let val = self.rol(acc);
            self.reg_a = val;

            println!("{:04X}  {:02X}         ROL A       {:?}", self.instr_pc, self.opcode, self);
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

        // TODO - RW ops should take 5 cycles
        if self.is_zero_page(self.opcode) && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.opcode == 0x05 && self.time == 3 {
            let mask = interconnect.read_byte(self.fetch as u16);
            self.ora(mask);

            println!("{:04X}  {:02X} {:02X}      ORA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, mask, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x06 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            let val = self.asl(val);
            interconnect.write_byte(self.fetch as u16, val);

            println!("{:04X}  {:02X} {:02X}      ASL ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x24 && self.time == 3 {
            let mask = interconnect.read_byte(self.fetch as u16);
            self.bit(mask);

            println!("{:04X}  {:02X} {:02X}      BIT ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, mask, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x25 && self.time == 3 {
            let mask = interconnect.read_byte(self.fetch as u16);
            self.reg_a &= mask;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      AND ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, mask, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x26 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            let val = self.rol(val);
            interconnect.write_byte(self.fetch as u16, val);
            println!("{:04X}  {:02X} {:02X}      ROL ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x45 && self.time == 3 {
            let mask = interconnect.read_byte(self.fetch as u16);
            self.reg_a ^= mask;

            self.reg_status.zero = self.reg_a == 0;
            self.reg_status.negative = (self.reg_a & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      EOR ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, mask, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x46 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            let val = self.lsr(val);
            interconnect.write_byte(self.fetch as u16, val);

            println!("{:04X}  {:02X} {:02X}      LSR ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x65 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.adc(val);
            println!("{:04X}  {:02X} {:02X}      ADC ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x66 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            let val = self.ror(val);
            interconnect.write_byte(self.fetch as u16, val);
            println!("{:04X}  {:02X} {:02X}      ROR ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x84 && self.time == 3 {
            interconnect.write_byte(self.fetch as u16, self.reg_y);
            println!("{:04X}  {:02X} {:02X}      STY ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_y, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x85 && self.time == 3 {
            interconnect.write_byte(self.fetch as u16, self.reg_a);
            println!("{:04X}  {:02X} {:02X}      STA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_a, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0x86 && self.time == 3 {
            interconnect.write_byte(self.fetch as u16, self.reg_x);
            println!("{:04X}  {:02X} {:02X}      STX ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_x, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xa4 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.ldy(val);

            println!("{:04X}  {:02X} {:02X}      LDY ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_y, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xa5 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.lda(val);
            println!("{:04X}  {:02X} {:02X}      LDA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_a, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xa6 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.reg_x = val;
            self.reg_status.zero = val == 0;
            self.reg_status.negative = (val & (1 << 7)) != 0;
            println!("{:04X}  {:02X} {:02X}      LDX ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, self.reg_x, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xc4 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.cpy(val);

            println!("{:04X}  {:02X} {:02X}      CPY ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xc5 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.cmp(val);
            println!("{:04X}  {:02X} {:02X}      CMP ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xc6 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            let val = self.dec(val);
            interconnect.write_byte(self.fetch as u16, val);
            println!("{:04X}  {:02X} {:02X}      DEC ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xe4 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            self.cpx(val);
            println!("{:04X}  {:02X} {:02X}      CPX ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xe5 && self.time == 3 {
            let val = !interconnect.read_byte(self.fetch as u16);
            self.adc(val);
            println!("{:04X}  {:02X} {:02X}      SBC ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xe6 && self.time == 3 {
            let val = interconnect.read_byte(self.fetch as u16);
            let val = self.inc(val);
            interconnect.write_byte(self.fetch as u16, val);
            println!("{:04X}  {:02X} {:02X}      INC ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.fetch, val, self);
            self.time = 0;
            return;
        }

        // Zero Page Indexed addressing
        if self.is_zero_indexed(self.opcode) && self.time == 2 {
            self.address = interconnect.read_byte(self.reg_pc) as u16;
            self.reg_pc = self.reg_pc.wrapping_add(1);
            return
        }
        if self.is_zero_indexed(self.opcode) && self.time == 3 {
            self.address = self.address.wrapping_add(self.reg_x as u16);
            self.address &= 0xff;
            return
        }
        // ORA zero page indexed
        if self.opcode == 0x15 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.ora(value);

            println!("{:04X}  {:02X} {:02X}      ORA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // AND zero page indexed
        if self.opcode == 0x35 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.and(value);

            println!("{:04X}  {:02X} {:02X}      AND ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // EOR zero page indexed
        if self.opcode == 0x55 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.eor(value);

            println!("{:04X}  {:02X} {:02X}      EOR ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // LSR zero page indexed
        if self.opcode == 0x56 && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0x56 && self.time == 5 {
            let value = self.fetch;
            self.fetch = self.lsr(value);
            return;
        }
        if self.opcode == 0x56 && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);

            println!("{:04X}  {:02X} {:02X}      LSR ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, self.fetch, self);
            self.time = 0;
            return;
        }
        // ADC zero page indexed
        if self.opcode == 0x75 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.adc(value);

            println!("{:04X}  {:02X} {:02X}      ADC ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // STY zero page indexed
        if self.opcode == 0x94 && self.time == 4 {
            interconnect.write_byte(self.address, self.reg_y);

            println!("{:04X}  {:02X} {:02X}      STY ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, self.reg_y, self);
            self.time = 0;
            return;
        }
        // STA zero page indexed
        if self.opcode == 0x95 && self.time == 4 {
            interconnect.write_byte(self.address, self.reg_a);

            println!("{:04X}  {:02X} {:02X}      STA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, self.reg_a, self);
            self.time = 0;
            return;
        }
        // LDY zero page indexed
        if self.opcode == 0xb4 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.ldy(value);

            println!("{:04X}  {:02X} {:02X}      LDY ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // LDA zero page indexed
        if self.opcode == 0xb5 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.lda(value);

            println!("{:04X}  {:02X} {:02X}      LDA ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // CMP zero page indexed
        if self.opcode == 0xd5 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.cmp(value);

            println!("{:04X}  {:02X} {:02X}      CMP ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
            self.time = 0;
            return;
        }
        // SBC zero page indexed
        if self.opcode == 0xf5 && self.time == 4 {
            let value = interconnect.read_byte(self.address);
            self.adc(!value);

            println!("{:04X}  {:02X} {:02X}      SBC ${:02X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address as u8, value, self);
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
        // ORA Absolute
        if self.opcode == 0x0d && self.time == 4 {
            let mask = interconnect.read_byte(self.address);
            self.ora(mask);
            println!("{:04X}  {:02X} {:02X} {:02X}   ORA ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // ASL Absolute
        if self.opcode == 0x0e && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0x0e && self.time == 5 {
            let val = self.fetch;
            self.fetch = self.asl(val);
            return;
        }
        if self.opcode == 0x0e && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);
            println!("{:04X}  {:02X} {:02X} {:02X}   ASL ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, self.fetch, self);
            self.time = 0;
            return;
        }
        // BIT Absolute
        if self.opcode == 0x2c && self.time == 4 {
            let mask = interconnect.read_byte(self.address);
            self.bit(mask);
            println!("{:04X}  {:02X} {:02X} {:02X}   BIT ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // AND Absolute
        if self.opcode == 0x2d && self.time == 4 {
            let mask = interconnect.read_byte(self.address);
            self.and(mask);
            println!("{:04X}  {:02X} {:02X} {:02X}   AND ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // ROL Absolute
        if self.opcode == 0x2e && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0x2e && self.time == 5 {
            let val = self.fetch;
            self.fetch = self.rol(val);
            return;
        }
        if self.opcode == 0x2e && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);
            println!("{:04X}  {:02X} {:02X} {:02X}   ROL ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, self.fetch, self);
            self.time = 0;
            return;
        }
        // EOR Absolute
        if self.opcode == 0x4d && self.time == 4 {
            let mask = interconnect.read_byte(self.address);
            self.eor(mask);
            println!("{:04X}  {:02X} {:02X} {:02X}   EOR ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // LSR Absolute
        if self.opcode == 0x4e && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0x4e && self.time == 5 {
            let val = self.fetch;
            self.fetch = self.lsr(val);
            return;
        }
        if self.opcode == 0x4e && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);
            println!("{:04X}  {:02X} {:02X} {:02X}   LSR ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, self.fetch, self);
            self.time = 0;
            return;
        }
        // ADC Absolute
        if self.opcode == 0x6d && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.adc(val);
            println!("{:04X}  {:02X} {:02X} {:02X}   ADC ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // ROR Absolute
        if self.opcode == 0x6e && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0x6e && self.time == 5 {
            let val = self.fetch;
            self.fetch = self.ror(val);
            return;
        }
        if self.opcode == 0x6e && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);
            println!("{:04X}  {:02X} {:02X} {:02X}   ROR ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, self.fetch, self);
            self.time = 0;
            return;
        }
        // STY Absolute
        if self.opcode == 0x8c && self.time == 4 {
            interconnect.write_byte(self.address, self.reg_y);
            println!("{:04X}  {:02X} {:02X} {:02X}   STY ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_y, self);
            self.time = 0;
            return;
        }
        // STA Absolute
        if self.opcode == 0x8d && self.time == 4 {
            interconnect.write_byte(self.address, self.reg_a);
            println!("{:04X}  {:02X} {:02X} {:02X}   STA ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // STX Absolute
        if self.opcode == 0x8e && self.time == 4 {
            interconnect.write_byte(self.address, self.reg_x);
            println!("{:04X}  {:02X} {:02X} {:02X}   STX ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_x, self);
            self.time = 0;
            return;
        }
        // LDY Absolute
        if self.opcode == 0xac && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.ldy(val);
            println!("{:04X}  {:02X} {:02X} {:02X}   LDY ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_y, self);
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
            self.lda(val);
            println!("{:04X}  {:02X} {:02X} {:02X}   LDA ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // CPY Absolute
        if self.opcode == 0xcc && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.cpy(val);
            println!("{:04X}  {:02X} {:02X} {:02X}   CPY ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_y, self);
            self.time = 0;
            return;
        }
        // CMP Absolute
        if self.opcode == 0xcd && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.cmp(val);
            println!("{:04X}  {:02X} {:02X} {:02X}   CMP ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // DEC Absolute
        if self.opcode == 0xce && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0xce && self.time == 5 {
            let val = self.fetch;
            self.fetch = self.dec(val);
            return;
        }
        if self.opcode == 0xce && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);
            println!("{:04X}  {:02X} {:02X} {:02X}   DEC ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, self.fetch, self);
            self.time = 0;
            return;
        }
        // CPX Absolute
        if self.opcode == 0xec && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.cpx(val);
            println!("{:04X}  {:02X} {:02X} {:02X}   CPX ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_x, self);
            self.time = 0;
            return;
        }
        // SBC Absolute
        if self.opcode == 0xed && self.time == 4 {
            let val = interconnect.read_byte(self.address);
            self.adc(!val);
            println!("{:04X}  {:02X} {:02X} {:02X}   SBC ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.fetch, self.address >> 8, self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // INC Absolute
        if self.opcode == 0xee && self.time == 4 {
            self.fetch = interconnect.read_byte(self.address);
            return;
        }
        if self.opcode == 0xee && self.time == 5 {
            let val = self.fetch;
            self.fetch = self.inc(val);
            return;
        }
        if self.opcode == 0xee && self.time == 6 {
            interconnect.write_byte(self.address, self.fetch);
            println!("{:04X}  {:02X} {:02X} {:02X}   INC ${:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, self.fetch, self);
            self.time = 0;
            return;
        }

        // Absolute Indexed (Y) Addressing
        // TODO - These instructions should be 5 cycles if we cross a page boundary
        if self.is_absolute_y(self.opcode) && self.time == 2 {
            self.address = interconnect.read_byte(self.reg_pc) as u16;
            self.reg_pc += 1;
            return;
        }
        if self.is_absolute_y(self.opcode) && self.time == 3 {
            let addrhi = interconnect.read_byte(self.reg_pc) as u16;
            self.reg_pc += 1;
            self.address |= addrhi << 8;
            return;
        }
        // ORA absolute indexed y
        if self.opcode == 0x19 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.ora(value);

            println!("{:04X}  {:02X} {:02X} {:02X}   ORA ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // AND absolute indexed y
        if self.opcode == 0x39 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.and(value);

            println!("{:04X}  {:02X} {:02X} {:02X}   AND ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // EOR absolute indexed y
        if self.opcode == 0x59 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.eor(value);

            println!("{:04X}  {:02X} {:02X} {:02X}   EOR ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // ADC absolute indexed y
        if self.opcode == 0x79 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.adc(value);

            println!("{:04X}  {:02X} {:02X} {:02X}   ADC ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // STA absolute indexed y
        if self.opcode == 0x99 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            interconnect.write_byte(addr, self.reg_a);

            println!("{:04X}  {:02X} {:02X} {:02X}   LDA ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // LDA absolute indexed y
        if self.opcode == 0xb9 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.lda(value);

            println!("{:04X}  {:02X} {:02X} {:02X}   LDA ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // CMP absolute indexed y
        if self.opcode == 0xd9 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.cmp(value);

            println!("{:04X}  {:02X} {:02X} {:02X}   CMP ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }
        // SBC absolute indexed y
        if self.opcode == 0xf9 && self.time == 4 {
            let addr = self.address.wrapping_add(self.reg_y as u16);
            let value = interconnect.read_byte(addr);
            self.adc(!value);

            println!("{:04X}  {:02X} {:02X} {:02X}   SBC ${:04X},Y @{:04X} = {:02X}  {:?}", self.instr_pc, self.opcode, self.address as u8, self.address >> 8, self.address, addr, self.reg_a, self);
            self.time = 0;
            return;
        }

        // Indexed Indirect Addressing
        if self.is_indexed_indirect(self.opcode) && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            print!("{0:04X}  {1:02X} {2:02X}      {3} (${2:02X}, X) @ {2:02X} = ", self.instr_pc, self.opcode, self.fetch, self.opcode_name(self.opcode));
            return;
        }
        if self.is_indexed_indirect(self.opcode) && self.time == 3 {
            self.fetch = self.fetch.wrapping_add(self.reg_x);
            return;
        }
        if self.is_indexed_indirect(self.opcode) && self.time == 4 {
            let addr_lo = interconnect.read_byte(self.fetch as u16) as u16;
            self.address = addr_lo;
            return;
        }
        if self.is_indexed_indirect(self.opcode) && self.time == 5 {
            let addr_hi = interconnect.read_byte(self.fetch.wrapping_add(1) as u16) as u16;
            self.address |= addr_hi << 8;
            return;
        }
        // ORA
        if self.opcode == 0x01 && self.time == 6 {
            let mask = interconnect.read_byte(self.address);
            self.ora(mask);

            println!("{:04X} =  {:02X}       {:?}", self.address, mask, self);
            self.time = 0;
            return;
        }
        // AND
        if self.opcode == 0x21 && self.time == 6 {
            let mask = interconnect.read_byte(self.address);
            self.and(mask);
            println!("{:04X} =  {:02X}       {:?}", self.address, mask, self);
            self.time = 0;
            return;
        }
        // EOR
        if self.opcode == 0x41 && self.time == 6 {
            let mask = interconnect.read_byte(self.address);
            self.eor(mask);
            println!("{:04X} =  {:02X}       {:?}", self.address, mask, self);
            self.time = 0;
            return;
        }
        // ADC
        if self.opcode == 0x61 && self.time == 6 {
            let val = interconnect.read_byte(self.address);
            self.adc(val);

            println!("{:04X} = {:02X}        {:?}", self.address, val, self);
            self.time = 0;
            return;
        }
        // STA
        if self.opcode == 0x81 && self.time == 6 {
            interconnect.write_byte(self.address, self.reg_a);

            println!("{:04X} = {:02X}   {:?}", self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // LDA
        if self.opcode == 0xa1 && self.time == 6 {
            let value = interconnect.read_byte(self.address);
            self.lda(value);

            println!("{:04X} = {:02X}   {:?}", self.address, self.reg_a, self);
            self.time = 0;
            return;
        }
        // CMP
        if self.opcode == 0xc1 && self.time == 6 {
            let val = interconnect.read_byte(self.address);
            self.cmp(val);

            println!("{:04X} = {:02X}   {:?}", self.address, val, self);
            self.time = 0;
            return;
        }
        if self.opcode == 0xe1 && self.time == 6 {
            let val = !interconnect.read_byte(self.address);
            self.adc(val);

            println!("{:04X} = {:02X}        {:?}", self.address, val, self);
            self.time = 0;
            return;
        }

        // Indirect Indexed Addressing
        // TODO - these instructions should take an extra cycle if we cross a page boundary
        if self.is_indirect_indexed(self.opcode) && self.time == 2 {
            self.fetch = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            return;
        }
        if self.is_indirect_indexed(self.opcode) && self.time == 3 {
            self.address = interconnect.read_byte(self.fetch as u16) as u16;
            let val = self.fetch.wrapping_add(1);
            self.fetch = val;
            return;
        }
        if self.is_indirect_indexed(self.opcode) && self.time == 4 {
            let addrhi = interconnect.read_byte(self.fetch as u16) as u16;
            let address = (addrhi << 8) | self.address;
            self.address = address.wrapping_add(self.reg_y as u16);
            print!("{0:04X}  {1:02X} {2:02X}      {3} (${2:02X}, X) @ {2:02X} = ", self.instr_pc, self.opcode, self.fetch, self.opcode_name(self.opcode));
            return;
        }
        // ORA indirect indexed
        if self.opcode == 0x11 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.ora(value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // AND indirect indexed
        if self.opcode == 0x31 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.and(value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // EOR indirect indexed
        if self.opcode == 0x51 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.eor(value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // ADC indirect indexed
        if self.opcode == 0x71 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.adc(value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // LDA indirect indexed
        if self.opcode == 0xb1 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.lda(value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // CMP indirect indexed
        if self.opcode == 0xd1 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.cmp(value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // SBC indirect indexed
        if self.opcode == 0xf1 && self.time == 5 {
            let value = interconnect.read_byte(self.address);
            self.adc(!value);

            println!("{:04X} = {:02X}   {:?}", self.address, value, self);
            self.time = 0;
            return;
        }
        // STA indirect indexed
        if self.opcode == 0x91 && self.time == 5 {
            // This cycle is used to fix up the address if a page boundary was crossed
            return;
        }
        if self.opcode == 0x91 && self.time == 6 {
            interconnect.write_byte(self.address, self.reg_a);

            println!("{:04X} = {:02X}   {:?}", self.address, self.reg_a, self);
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
            self.eor(mask);
            println!("{:04X}  {:02X} {:02X}      EOR #${:02X}      {:?}", self.instr_pc, self.opcode, mask, mask, self);
            self.time = 0;
            return;
        }

        // SBC
        if self.opcode == 0xe9 && self.time == 2 {
            let val = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.adc(!val);
            println!("{:04X}  {:02X} {:02X}      {} #${:02X}      {:?}", self.instr_pc, self.opcode, val, self.opcode_name(self.opcode), val, self);
            self.time = 0;
            return;
        }
        // ADC
        if (self.opcode == 0x69) && self.time == 2 {
            let val = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.adc(val);
            println!("{:04X}  {:02X} {:02X}      {} #${:02X}      {:?}", self.instr_pc, self.opcode, val, self.opcode_name(self.opcode), val, self);
            self.time = 0;
            return;
        }

        // CPY
        if self.opcode == 0xc0 && self.time == 2 {
            let val = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.cpy(val);

            println!("{:04X}  {:02X} {:02X}      CPY #${:02X}      {:?}", self.instr_pc, self.opcode, val, val, self);
            self.time = 0;
            return;
        }
        // CMP
        if self.opcode == 0xc9 && self.time == 2 {
            let val = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.cmp(val);

            println!("{:04X}  {:02X} {:02X}      CMP #${:02X}      {:?}", self.instr_pc, self.opcode, val, val, self);
            self.time = 0;
            return;
        }
        // CPX
        if self.opcode == 0xe0 && self.time == 2 {
            let val = interconnect.read_byte(self.reg_pc);
            self.reg_pc += 1;
            self.cpx(val);
            println!("{:04X}  {:02X} {:02X}      CPX #${:02X}      {:?}", self.instr_pc, self.opcode, val, val, self);
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

    fn is_absolute_y(&self, opcode: u8) -> bool {
        opcode & 25 == 25 && !opcode & 6 == 6
    }

    fn is_zero_page(&self, opcode: u8) -> bool {
        opcode & 4 == 4 && !opcode & 24 == 24
    }

    fn is_zero_indexed(&self, opcode: u8) -> bool {
        (opcode & 20 == 20 && !opcode & 10 == 10) || (opcode == 0x56)
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
