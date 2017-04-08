use interconnect::Interconnect;

#[derive(Clone, Copy)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        let mut ret = 0;
        if self.c {
            ret |= 1 << 4;
        }
        if self.h {
            ret |= 1 << 5;
        }
        if self.n {
            ret |= 1 << 6;
        }
        if self.z {
            ret |= 1 << 7;
        }
        ret
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            z: (value & (1 << 7)) != 0,
            n: (value & (1 << 6)) != 0,
            h: (value & (1 << 5)) != 0,
            c: (value & (1 << 4)) != 0,
        }
    }
}

pub struct Cpu {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,

    pub instructions_to_di: u8,
    pub interrupts_enabled: bool,

    pub halted: i8,

    pub total_cycles: u32,
}

#[cfg_attr(rustfmt, rustfmt_skip)]
static CYCLE_COUNTS: [u16; 256] = [
     4, 12,  8,  8,  4,  4,  8,  4, 20,  8,  8,  8,  4,  4,  8,  4,
     0, 12,  8,  8,  4,  4,  8,  4, 12,  8,  8,  8,  4,  4,  8,  4,
     8, 12,  8,  8,  4,  4,  8,  4,  8,  8,  8,  8,  4,  4,  8,  4,
     8, 12,  8,  8, 12, 12, 12,  4,  8,  8,  8,  8,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     8,  8,  8,  8,  8,  8,  0,  8,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     4,  4,  4,  4,  4,  4,  8,  4,  4,  4,  4,  4,  4,  4,  8,  4,
     8, 12, 12, 16, 12, 16,  8, 16,  8, 16, 12,  0, 12, 24,  8, 16,
     8, 12, 12,  0, 12, 16,  8, 16,  8, 16, 12,  0, 12,  0,  8, 16,
    12, 12,  8,  0,  0, 16,  8, 16, 16,  4, 16,  0,  0,  0,  8, 16,
    12, 12,  8,  4,  0, 16,  8, 16, 12,  8, 16,  4,  0,  0,  8, 16
];

#[cfg_attr(rustfmt, rustfmt_skip)]
static CB_CYCLE_COUNTS: [u16; 256] = [
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 12,  8,  8,  8,  8,  8,  8,  8, 12,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8,
     8,  8,  8,  8,  8,  8, 16,  8,  8,  8,  8,  8,  8,  8, 16,  8
];

impl Cpu {
    pub fn new() -> Cpu {
        let f = Flags {
            z: false,
            n: false,
            h: false,
            c: false,
        };

        Cpu {
            a: 0,
            f: f,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            sp: 0xfffe,
            pc: 0x100,

            instructions_to_di: 0,
            interrupts_enabled: true,

            halted: 0,

            total_cycles: 0,
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms, cyclomatic_complexity))]
    pub fn step(&mut self, interconnect: &mut Interconnect) -> u16 {
        let interrupt_flags = interconnect.read_byte(0xff0f);
        let interrupt_enable = interconnect.read_byte(0xffff);
        let interrupt_request = interrupt_flags & interrupt_enable;

        if self.halted == 1 && interrupt_request == 0 {
            // Step forward one NOP
            return 4;
        }

        if self.halted == 1 && !self.interrupts_enabled {
            self.halted = 0;
        }

        if self.interrupts_enabled && (interrupt_request != 0) {
            self.handle_interrupt(interconnect, interrupt_flags, interrupt_enable);
        }

        let old_pc = self.pc;
        let instr = self.read_pc_byte(interconnect);
        let mut cycle_count = CYCLE_COUNTS[instr as usize];

        match instr {
            0x00 => {} // NOP - No Operation
            0x01 => {
                // LD BC, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.b = msb;
                self.c = lsb;
            }
            0x02 => interconnect.write_byte(self.bc(), self.a), // LD (BC), A
            0x03 => {
                // INC BC
                let bc = self.bc().wrapping_add(1);
                self.set_bc(bc);
            }
            0x04 => {
                let val = self.b;
                self.b = self.inc(val);
            }
            0x05 => {
                let val = self.b;
                self.b = self.dec(val);
            }
            0x06 => self.b = self.read_pc_byte(interconnect), // LD B,n
            0x07 => {
                let val = self.a;
                self.a = self.rlc(val);
                self.f.z = false;
            }
            0x08 => {
                // LD (nn), SP
                let addr = self.read_pc_halfword(interconnect);

                interconnect.write_halfword(addr, self.sp);
            }
            0x09 => {
                // ADD HL, BC
                let (hl, bc) = (self.hl(), self.bc());
                let val = self.add16(hl, bc);

                self.set_hl(val);
            }
            0x0a => {
                let addr = self.bc();
                self.a = interconnect.read_byte(addr);
            }
            0x0b => {
                // DEC BC
                let bc = self.bc().wrapping_sub(1);
                self.set_bc(bc);
            }
            0x0c => {
                let val = self.c;
                self.c = self.inc(val);
            }
            0x0d => {
                let val = self.c;
                self.c = self.dec(val);
            }
            0x0e => self.c = self.read_pc_byte(interconnect), // LD C,n
            0x0f => {
                // RRC A
                let val = self.a;
                self.a = self.rrc(val);
                self.f.z = false;
            }
            0x11 => {
                // LD DE, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.d = msb;
                self.e = lsb;
            }
            0x12 => interconnect.write_byte(self.de(), self.a), // LD (DE), A
            0x13 => {
                // INC DE
                let de = self.de().wrapping_add(1);
                self.set_de(de);
            }
            0x14 => {
                let val = self.d;
                self.d = self.inc(val);
            }
            0x15 => {
                let val = self.d;
                self.d = self.dec(val);
            }
            0x16 => self.d = self.read_pc_byte(interconnect), // LD D,n
            0x17 => {
                let val = self.a;
                self.a = self.rl(val);
                self.f.z = false;
            }
            0x18 => {
                // JR n - realtive jump by n
                let n = self.read_pc_byte(interconnect);
                self.pc = self.pc.wrapping_add(n as i8 as u16);
            }
            0x19 => {
                // ADD HL, DE
                let (hl, de) = (self.hl(), self.de());
                let val = self.add16(hl, de);

                self.set_hl(val);
            }
            0x1a => self.a = interconnect.read_byte(self.de()),
            0x1b => {
                // DEC DE
                let de = self.de().wrapping_sub(1);
                self.set_de(de);
            }
            0x1c => {
                let val = self.e;
                self.e = self.inc(val);
            }
            0x1d => {
                let val = self.e;
                self.e = self.dec(val);
            }
            0x1e => self.e = self.read_pc_byte(interconnect), // LD E,n
            0x1f => {
                // RR A
                let val = self.a;
                self.a = self.rr(val);
                self.f.z = false;
            }
            0x20 => {
                // JR NZ, n
                let n = self.read_pc_byte(interconnect) as i8 as u16;

                if !self.f.z {
                    self.pc = self.pc.wrapping_add(n);
                    cycle_count += 4;
                }
            }
            0x21 => {
                // LD HL, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.h = msb;
                self.l = lsb;
            }
            0x22 => {
                // LDI (HL), A
                interconnect.write_byte(self.hl(), self.a);
                let val = self.hl().wrapping_add(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x23 => {
                // INC DE
                let hl = self.hl().wrapping_add(1);
                self.set_hl(hl);
            }
            0x24 => {
                let val = self.h;
                self.h = self.inc(val);
            }
            0x25 => {
                let val = self.h;
                self.h = self.dec(val);
            }
            0x26 => self.h = self.read_pc_byte(interconnect), // LD H,n
            0x27 => {
                // DAA - Decimal adjust a
                let mut val = self.a as u8;

                if !self.f.n {
                    if self.f.h || (val & 0xf) > 9 {
                        let (res, overflow) = val.overflowing_add(0x06);
                        self.f.c |= overflow;
                        val = res;
                    }
                    if self.f.c || (val > 0x9f) {
                        let (res, overflow) = val.overflowing_add(0x60);
                        self.f.c |= overflow;
                        val = res;
                    }
                } else {
                    if self.f.h {
                        val = val.wrapping_sub(0x06);
                    }
                    if self.f.c {
                        val = val.wrapping_sub(0x60);
                    }
                }

                self.f.h = false;
                self.f.z = val == 0;

                self.a = (val & 0xff) as u8;
            }
            0x28 => {
                // JR Z, n
                let n = self.read_pc_byte(interconnect) as i8 as u16;

                if self.f.z {
                    self.pc = self.pc.wrapping_add(n);
                    cycle_count += 4;
                }
            }
            0x29 => {
                // ADD HL, HL
                let hl = self.hl();
                let val = self.add16(hl, hl);

                self.set_hl(val);
            }
            0x2a => {
                // LDI A, (HL) - Load the value at address HL into A, increment HL
                self.a = interconnect.read_byte(self.hl());
                let val = self.hl().wrapping_add(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x2b => {
                // DEC HL
                let hl = self.hl().wrapping_sub(1);
                self.set_hl(hl);
            }
            0x2c => {
                let val = self.l;
                self.l = self.inc(val);
            }
            0x2d => {
                let val = self.l;
                self.l = self.dec(val);
            }
            0x2e => self.l = self.read_pc_byte(interconnect), // LD L,n
            0x2f => {
                self.a = !self.a;

                self.f.n = true;
                self.f.h = true;
            }
            0x30 => {
                // JMP NC, n
                let n = self.read_pc_byte(interconnect) as i8 as u16;

                if !self.f.c {
                    self.pc = self.pc.wrapping_add(n);
                    cycle_count += 4;
                }
            }
            0x31 => {
                // LD SP, nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                let val = ((msb as u16) << 8) | lsb as u16;

                self.sp = val;
            }
            0x32 => {
                // LDD (HL), A
                interconnect.write_byte(self.hl(), self.a);
                let val = self.hl().wrapping_sub(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x33 => {
                // INC SP
                let sp = self.sp.wrapping_add(1);
                self.sp = sp;
            }
            0x34 => {
                let val = interconnect.read_byte(self.hl());
                interconnect.write_byte(self.hl(), self.inc(val));
            }
            0x35 => {
                let val = interconnect.read_byte(self.hl());
                interconnect.write_byte(self.hl(), self.dec(val));
            }
            0x36 => {
                let val = self.read_pc_byte(interconnect);

                interconnect.write_byte(self.hl(), val);
            }
            0x37 => {
                // SCF
                self.f.n = false;
                self.f.h = false;
                self.f.c = true;
            }
            0x38 => {
                // JMP C, n
                let n = self.read_pc_byte(interconnect) as i8 as u16;

                if self.f.c {
                    self.pc = self.pc.wrapping_add(n);
                    cycle_count += 4;
                }
            }
            0x39 => {
                // ADD HL, SP
                let (hl, sp) = (self.hl(), self.sp);
                let val = self.add16(hl, sp);

                self.set_hl(val);
            }
            0x3a => {
                self.a = interconnect.read_byte(self.hl());
                let val = self.hl().wrapping_sub(1);

                self.h = (val >> 8) as u8;
                self.l = (val & 0xff) as u8;
            }
            0x3b => {
                // DEC SP
                let sp = self.sp.wrapping_sub(1);
                self.sp = sp;
            }
            0x3c => {
                let val = self.a;
                self.a = self.inc(val);
            }
            0x3d => {
                let val = self.a;
                self.a = self.dec(val);
            }
            0x3e => {
                // LD A, # - Load immediate 8-bit into A
                let val = self.read_pc_byte(interconnect);

                self.a = val;
            }
            0x3f => {
                // CCF - complement carry flag
                self.f.n = false;
                self.f.h = false;
                self.f.c = !self.f.c;
            }
            0x40 => {} // LD B, B
            0x41 => self.b = self.c, // LD B, C
            0x42 => self.b = self.d, // LD B, D
            0x43 => self.b = self.e, // LD B, E
            0x44 => self.b = self.h, // LD B, H
            0x45 => self.b = self.l, // LD B, L
            0x46 => self.b = interconnect.read_byte(self.hl()), // LD B, (HL)
            0x47 => self.b = self.a, // LD B, A
            0x48 => self.c = self.b, // LD C, B
            0x49 => {} // LD C, C
            0x4a => self.c = self.d, // LD C, D
            0x4b => self.c = self.e, // LD C, E
            0x4c => self.c = self.h, // LD C, H
            0x4d => self.c = self.l, // LD C, L
            0x4e => self.c = interconnect.read_byte(self.hl()), // LD C, (HL)
            0x4f => self.c = self.a, // LD C, A
            0x50 => self.d = self.b, // LD D, B
            0x51 => self.d = self.c, // LD D, C
            0x52 => {} // LD D, D
            0x53 => self.d = self.e, // LD D, E
            0x54 => self.d = self.h, // LD D, H
            0x55 => self.d = self.l, // LD D, L
            0x56 => self.d = interconnect.read_byte(self.hl()), // LD D, (HL)
            0x57 => self.d = self.a, // LD D, A
            0x58 => self.e = self.b, // LD E, B
            0x59 => self.e = self.c, // LD E, C
            0x5a => self.e = self.d, // LD E, D
            0x5b => {} // LD E, E
            0x5c => self.e = self.h, // LD E, H
            0x5d => self.e = self.l, // LD E, L
            0x5e => self.e = interconnect.read_byte(self.hl()), // LD E, (HL)
            0x5f => self.e = self.a, // LD E, A
            0x60 => self.h = self.b, // LD H, B
            0x61 => self.h = self.c, // LD H, C
            0x62 => self.h = self.d, // LD H, D
            0x63 => self.h = self.e, // LD H, E
            0x64 => {} // LD H, H
            0x65 => self.h = self.l, // LD H, L
            0x66 => self.h = interconnect.read_byte(self.hl()), // LD H, (HL)
            0x67 => self.h = self.a, // LD H, A
            0x68 => self.l = self.b, // LD L, B
            0x69 => self.l = self.c, // LD L, C
            0x6a => self.l = self.d, // LD L, D
            0x6b => self.l = self.e, // LD L, E
            0x6c => self.l = self.h, // LD L, H
            0x6f => self.l = self.a, // LD L, A
            0x6d => {} // LD L, L
            0x6e => self.l = interconnect.read_byte(self.hl()), // LD L, (HL)
            0x70 => interconnect.write_byte(self.hl(), self.b), // LD (HL), B
            0x71 => interconnect.write_byte(self.hl(), self.c), // LD (HL), C
            0x72 => interconnect.write_byte(self.hl(), self.d), // LD (HL), D
            0x73 => interconnect.write_byte(self.hl(), self.e), // LD (HL), E
            0x74 => interconnect.write_byte(self.hl(), self.h), // LD (HL), H
            0x75 => interconnect.write_byte(self.hl(), self.l), // LD (HL), L
            0x76 => {
                // HALT
                if !self.interrupts_enabled && interrupt_request != 0 {
                    self.halted = -1;
                } else {
                    self.halted = 1;
                }
            }
            0x77 => interconnect.write_byte(self.hl(), self.a), // LD (HL), A
            0x78 => self.a = self.b, // LD A, B
            0x79 => self.a = self.c, // LD A, C
            0x7a => self.a = self.d, // LD A, D
            0x7b => self.a = self.e, // LD A, E
            0x7c => self.a = self.h, // LD A, H
            0x7d => self.a = self.l, // LD A, L
            0x7e => {
                // LD A, (HL)
                let addr = self.hl();
                self.a = interconnect.read_byte(addr);
            }
            0x7f => {} // LD A, A
            0x80 => {
                // ADD A, B
                let val = self.b;
                self.a = self.addc(val, false);
            }
            0x81 => {
                // ADD A, C
                let val = self.c;
                self.a = self.addc(val, false);
            }
            0x82 => {
                // ADD A, D
                let val = self.d;
                self.a = self.addc(val, false);
            }
            0x83 => {
                // ADD A, E
                let val = self.e;
                self.a = self.addc(val, false);
            }
            0x84 => {
                // ADD A, H
                let val = self.h;
                self.a = self.addc(val, false);
            }
            0x85 => {
                // ADD A, L
                let val = self.l;
                self.a = self.addc(val, false);
            }
            0x86 => {
                // ADD A, (HL)
                let val = interconnect.read_byte(self.hl());
                self.a = self.addc(val, false);
            }
            0x87 => {
                // ADD A, A
                let val = self.a;
                self.a = self.addc(val, false);
            }
            0x88 => {
                // ADDC A, B
                let val = self.b;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x89 => {
                // ADDC A, C
                let val = self.c;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x8a => {
                // ADDC A, D
                let val = self.d;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x8b => {
                // ADDC A, E
                let val = self.e;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x8c => {
                // ADDC A, H
                let val = self.h;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x8d => {
                // ADDC A, L
                let val = self.l;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x8e => {
                // ADDC A, (HL)
                let val = interconnect.read_byte(self.hl());
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x8f => {
                // ADDC A, A
                let val = self.a;
                let carry = self.f.c;
                self.a = self.addc(val, carry);
            }
            0x90 => {
                // SUB A, B
                let val = self.b;
                self.a = self.subc(val, false);
            }
            0x91 => {
                // SUB A, C
                let val = self.c;
                self.a = self.subc(val, false);
            }
            0x92 => {
                // SUB A, D
                let val = self.d;
                self.a = self.subc(val, false);
            }
            0x93 => {
                // SUB A, E
                let val = self.e;
                self.a = self.subc(val, false);
            }
            0x94 => {
                // SUB A, H
                let val = self.h;
                self.a = self.subc(val, false);
            }
            0x95 => {
                // SUB A, L
                let val = self.l;
                self.a = self.subc(val, false);
            }
            0x96 => {
                // SUB A, (HL)
                let val = interconnect.read_byte(self.hl());
                self.a = self.subc(val, false);
            }
            0x97 => {
                // SUB A, A
                let val = self.a;
                self.a = self.subc(val, false);
            }
            0x98 => {
                // SUBC A, B
                let val = self.b;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x99 => {
                // SUBC A, C
                let val = self.c;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x9a => {
                // SUBC A, D
                let val = self.d;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x9b => {
                // SUBC A, E
                let val = self.e;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x9c => {
                // SUBC A, H
                let val = self.h;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x9d => {
                // SUBC A, L
                let val = self.l;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x9e => {
                // SUBC A, (HL)
                let val = interconnect.read_byte(self.hl());
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0x9f => {
                // SUBC A, A
                let val = self.a;
                let carry = self.f.c;
                self.a = self.subc(val, carry);
            }
            0xa0 => {
                let val = self.b;
                self.a = self.and(val);
            }
            0xa1 => {
                let val = self.c;
                self.a = self.and(val);
            }
            0xa2 => {
                let val = self.d;
                self.a = self.and(val);
            }
            0xa3 => {
                let val = self.e;
                self.a = self.and(val);
            }
            0xa4 => {
                let val = self.h;
                self.a = self.and(val);
            }
            0xa5 => {
                let val = self.l;
                self.a = self.and(val);
            }
            0xa6 => {
                let val = interconnect.read_byte(self.hl());
                self.a = self.and(val);
            }
            0xa7 => {
                let val = self.a;
                self.a = self.and(val);
            }
            0xa8 => {
                let val = self.b;
                self.a = self.xor(val);
            }
            0xa9 => {
                let val = self.c;
                self.a = self.xor(val);
            }
            0xaa => {
                let val = self.d;
                self.a = self.xor(val);
            }
            0xab => {
                let val = self.e;
                self.a = self.xor(val);
            }
            0xac => {
                let val = self.h;
                self.a = self.xor(val);
            }
            0xad => {
                let val = self.l;
                self.a = self.xor(val);
            }
            0xae => {
                let val = interconnect.read_byte(self.hl());
                self.a = self.xor(val);
            }
            0xaf => {
                let val = self.a;
                self.a = self.xor(val);
            }
            0xb0 => {
                let val = self.b;
                self.a = self.or(val);
            }
            0xb1 => {
                let val = self.c;
                self.a = self.or(val);
            }
            0xb2 => {
                let val = self.d;
                self.a = self.or(val);
            }
            0xb3 => {
                let val = self.e;
                self.a = self.or(val);
            }
            0xb4 => {
                let val = self.h;
                self.a = self.or(val);
            }
            0xb5 => {
                let val = self.l;
                self.a = self.or(val);
            }
            0xb6 => {
                let val = interconnect.read_byte(self.hl());
                self.a = self.or(val);
            }
            0xb7 => {
                let val = self.a;
                self.a = self.or(val);
            }
            0xb8 => {
                let val = self.b;
                self.subc(val, false);
            }
            0xb9 => {
                let val = self.c;
                self.subc(val, false);
            }
            0xba => {
                let val = self.d;
                self.subc(val, false);
            }
            0xbb => {
                let val = self.e;
                self.subc(val, false);
            }
            0xbc => {
                let val = self.h;
                self.subc(val, false);
            }
            0xbd => {
                let val = self.l;
                self.subc(val, false);
            }
            0xbe => {
                let val = interconnect.read_byte(self.hl());
                self.subc(val, false);
            }
            0xbf => {
                let val = self.a;
                self.subc(val, false);
            }
            0xc0 => {
                // RET NZ - return if NZ
                if !self.f.z {
                    self.ret(interconnect);
                    cycle_count += 12;
                }
            }
            0xc1 => {
                // POP BC
                let c = self.pop_byte(interconnect);
                let b = self.pop_byte(interconnect);

                self.b = b;
                self.c = c;
            }
            0xc2 => {
                // JP NZ, nn - Jump to address nn if NZ
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                if !self.f.z {
                    self.pc = ((msb as u16) << 8) | lsb as u16;
                    cycle_count += 4;
                }
            }
            0xc3 => {
                // JP nn - Jump to address nn
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                self.pc = ((msb as u16) << 8) | lsb as u16;
            }
            0xc4 => {
                // CALL NZ, nn
                let addr = self.read_pc_halfword(interconnect);

                if !self.f.z {
                    self.call(interconnect, addr);
                    cycle_count += 12;
                }
            }
            0xc5 => {
                // PUSH BC
                let halfword = self.bc();
                self.push_halfword(interconnect, halfword);
            }
            0xc6 => {
                let n = self.read_pc_byte(interconnect);
                self.a = self.addc(n, false);
            }
            0xc7 => {
                self.call(interconnect, 0x0000);
            }
            0xc8 => {
                // RET Z - return if Z flag is set
                if self.f.z {
                    self.ret(interconnect);
                    cycle_count += 12;
                }
            }
            0xc9 => {
                // RET - pop return address and jump there
                self.ret(interconnect);
            }
            0xca => {
                // JP Z, nn - Jump to address nn if Z
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                if self.f.z {
                    self.pc = ((msb as u16) << 8) | lsb as u16;
                    cycle_count += 4;
                }
            }
            0xcb => {
                // Extended instructions
                let sub_instr = self.read_pc_byte(interconnect);
                match sub_instr {
                    0x00 => {
                        let val = self.b;
                        self.b = self.rlc(val);
                    }
                    0x01 => {
                        let val = self.c;
                        self.c = self.rlc(val);
                    }
                    0x02 => {
                        let val = self.d;
                        self.d = self.rlc(val);
                    }
                    0x03 => {
                        let val = self.e;
                        self.e = self.rlc(val);
                    }
                    0x04 => {
                        let val = self.h;
                        self.h = self.rlc(val);
                    }
                    0x05 => {
                        let val = self.l;
                        self.l = self.rlc(val);
                    }
                    0x06 => {
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.rlc(val));
                    }
                    0x07 => {
                        let val = self.a;
                        self.a = self.rlc(val);
                    }
                    0x08 => {
                        let val = self.b;
                        self.b = self.rrc(val);
                    }
                    0x09 => {
                        let val = self.c;
                        self.c = self.rrc(val);
                    }
                    0x0a => {
                        let val = self.d;
                        self.d = self.rrc(val);
                    }
                    0x0b => {
                        let val = self.e;
                        self.e = self.rrc(val);
                    }
                    0x0c => {
                        let val = self.h;
                        self.h = self.rrc(val);
                    }
                    0x0d => {
                        let val = self.l;
                        self.l = self.rrc(val);
                    }
                    0x0e => {
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.rrc(val));
                    }
                    0x0f => {
                        let val = self.a;
                        self.a = self.rrc(val);
                    }
                    0x10 => {
                        let val = self.b;
                        self.b = self.rl(val);
                    }
                    0x11 => {
                        let val = self.c;
                        self.c = self.rl(val);
                    }
                    0x12 => {
                        let val = self.d;
                        self.d = self.rl(val);
                    }
                    0x13 => {
                        let val = self.e;
                        self.e = self.rl(val);
                    }
                    0x14 => {
                        let val = self.h;
                        self.h = self.rl(val);
                    }
                    0x15 => {
                        let val = self.l;
                        self.l = self.rl(val);
                    }
                    0x16 => {
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.rl(val));
                    }
                    0x17 => {
                        let val = self.a;
                        self.a = self.rl(val);
                    }
                    0x18 => {
                        let val = self.b;
                        self.b = self.rr(val);
                    }
                    0x19 => {
                        let val = self.c;
                        self.c = self.rr(val);
                    }
                    0x1a => {
                        let val = self.d;
                        self.d = self.rr(val);
                    }
                    0x1b => {
                        let val = self.e;
                        self.e = self.rr(val);
                    }
                    0x1c => {
                        let val = self.h;
                        self.h = self.rr(val);
                    }
                    0x1d => {
                        let val = self.l;
                        self.l = self.rr(val);
                    }
                    0x1e => {
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.rr(val));
                    }
                    0x1f => {
                        let val = self.a;
                        self.a = self.rr(val);
                    }
                    0x20 => {
                        let val = self.b;
                        self.b = self.sla(val);
                    }
                    0x21 => {
                        let val = self.c;
                        self.c = self.sla(val);
                    }
                    0x22 => {
                        let val = self.d;
                        self.d = self.sla(val);
                    }
                    0x23 => {
                        let val = self.e;
                        self.e = self.sla(val);
                    }
                    0x24 => {
                        let val = self.h;
                        self.h = self.sla(val);
                    }
                    0x25 => {
                        let val = self.l;
                        self.l = self.sla(val);
                    }
                    0x26 => {
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.sla(val));
                    }
                    0x27 => {
                        let val = self.a;
                        self.a = self.sla(val);
                    }
                    0x28 => {
                        let val = self.b;
                        self.b = self.sra(val);
                    }
                    0x29 => {
                        let val = self.c;
                        self.c = self.sra(val);
                    }
                    0x2a => {
                        let val = self.d;
                        self.d = self.sra(val);
                    }
                    0x2b => {
                        let val = self.e;
                        self.e = self.sra(val);
                    }
                    0x2c => {
                        let val = self.h;
                        self.h = self.sra(val);
                    }
                    0x2d => {
                        let val = self.l;
                        self.l = self.sra(val);
                    }
                    0x2e => {
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.sra(val));
                    }
                    0x2f => {
                        let val = self.a;
                        self.a = self.sra(val);
                    }
                    0x30 => {
                        // SWAP B
                        let val = self.b;
                        self.b = self.swap(val);
                    }
                    0x31 => {
                        // SWAP C
                        let val = self.c;
                        self.c = self.swap(val);
                    }
                    0x32 => {
                        // SWAP D
                        let val = self.d;
                        self.d = self.swap(val);
                    }
                    0x33 => {
                        // SWAP E
                        let val = self.e;
                        self.e = self.swap(val);
                    }
                    0x34 => {
                        // SWAP H
                        let val = self.h;
                        self.h = self.swap(val);
                    }
                    0x35 => {
                        // SWAP L
                        let val = self.l;
                        self.l = self.swap(val);
                    }
                    0x36 => {
                        // SWAP (HL)
                        let val = interconnect.read_byte(self.hl());
                        let res = self.swap(val);
                        interconnect.write_byte(self.hl(), res);
                    }
                    0x37 => {
                        // SWAP A
                        let val = self.a;
                        self.a = self.swap(val);
                    }
                    0x38 => {
                        // SRL B
                        let val = self.b;
                        self.b = self.srl(val);
                    }
                    0x39 => {
                        // SRL C
                        let val = self.c;
                        self.c = self.srl(val);
                    }
                    0x3a => {
                        // SRL D
                        let val = self.d;
                        self.d = self.srl(val);
                    }
                    0x3b => {
                        // SRL E
                        let val = self.e;
                        self.e = self.srl(val);
                    }
                    0x3c => {
                        // SRL H
                        let val = self.h;
                        self.h = self.srl(val);
                    }
                    0x3d => {
                        // SRL L
                        let val = self.l;
                        self.l = self.srl(val);
                    }
                    0x3e => {
                        // SRL (HL)
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.srl(val));
                    }
                    0x3f => {
                        // SRL A
                        let val = self.a;
                        self.a = self.srl(val);
                    }
                    0x40 => {
                        // BIT B, 0
                        let val = self.b;
                        self.bit(val, 0);
                    }
                    0x41 => {
                        // BIT C, 0
                        let val = self.c;
                        self.bit(val, 0);
                    }
                    0x42 => {
                        // BIT D, 0
                        let val = self.d;
                        self.bit(val, 0);
                    }
                    0x43 => {
                        // BIT E, 0
                        let val = self.e;
                        self.bit(val, 0);
                    }
                    0x44 => {
                        // BIT H, 0
                        let val = self.h;
                        self.bit(val, 0);
                    }
                    0x45 => {
                        // BIT L, 0
                        let val = self.l;
                        self.bit(val, 0);
                    }
                    0x46 => {
                        // BIT (HL), 0
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 0);
                    }
                    0x47 => {
                        // BIT A, 0
                        let val = self.a;
                        self.bit(val, 0);
                    }
                    0x48 => {
                        // BIT B, 1
                        let val = self.b;
                        self.bit(val, 1);
                    }
                    0x49 => {
                        // BIT C, 1
                        let val = self.c;
                        self.bit(val, 1);
                    }
                    0x4a => {
                        // BIT D, 1
                        let val = self.d;
                        self.bit(val, 1);
                    }
                    0x4b => {
                        // BIT E, 1
                        let val = self.e;
                        self.bit(val, 1);
                    }
                    0x4c => {
                        // BIT H, 1
                        let val = self.h;
                        self.bit(val, 1);
                    }
                    0x4d => {
                        // BIT L, 1
                        let val = self.l;
                        self.bit(val, 1);
                    }
                    0x4e => {
                        // BIT (HL), 1
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 1);
                    }
                    0x4f => {
                        // BIT A, 1
                        let val = self.a;
                        self.bit(val, 1);
                    }
                    0x50 => {
                        // BIT B, 2
                        let val = self.b;
                        self.bit(val, 2);
                    }
                    0x51 => {
                        // BIT C, 2
                        let val = self.c;
                        self.bit(val, 2);
                    }
                    0x52 => {
                        // BIT D, 2
                        let val = self.d;
                        self.bit(val, 2);
                    }
                    0x53 => {
                        // BIT E, 2
                        let val = self.e;
                        self.bit(val, 2);
                    }
                    0x54 => {
                        // BIT H, 2
                        let val = self.h;
                        self.bit(val, 2);
                    }
                    0x55 => {
                        // BIT L, 2
                        let val = self.l;
                        self.bit(val, 2);
                    }
                    0x56 => {
                        // BIT (HL), 2
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 2);
                    }
                    0x57 => {
                        // BIT A, 2
                        let val = self.a;
                        self.bit(val, 2);
                    }
                    0x58 => {
                        // BIT B, 3
                        let val = self.b;
                        self.bit(val, 3);
                    }
                    0x59 => {
                        // BIT C, 3
                        let val = self.c;
                        self.bit(val, 3);
                    }
                    0x5a => {
                        // BIT D, 3
                        let val = self.d;
                        self.bit(val, 3);
                    }
                    0x5b => {
                        // BIT E, 3
                        let val = self.e;
                        self.bit(val, 3);
                    }
                    0x5c => {
                        // BIT H, 3
                        let val = self.h;
                        self.bit(val, 3);
                    }
                    0x5d => {
                        // BIT L, 3
                        let val = self.l;
                        self.bit(val, 3);
                    }
                    0x5e => {
                        // BIT (HL), 3
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 3);
                    }
                    0x5f => {
                        // BIT A, 3
                        let val = self.a;
                        self.bit(val, 3);
                    }
                    0x60 => {
                        // BIT B, 4
                        let val = self.b;
                        self.bit(val, 4);
                    }
                    0x61 => {
                        // BIT C, 4
                        let val = self.c;
                        self.bit(val, 4);
                    }
                    0x62 => {
                        // BIT D, 4
                        let val = self.d;
                        self.bit(val, 4);
                    }
                    0x63 => {
                        // BIT E, 4
                        let val = self.e;
                        self.bit(val, 4);
                    }
                    0x64 => {
                        // BIT H, 4
                        let val = self.h;
                        self.bit(val, 4);
                    }
                    0x65 => {
                        // BIT L, 4
                        let val = self.l;
                        self.bit(val, 4);
                    }
                    0x66 => {
                        // BIT (HL), 4
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 4);
                    }
                    0x67 => {
                        // BIT A, 4
                        let val = self.a;
                        self.bit(val, 4);
                    }
                    0x68 => {
                        // BIT B, 5
                        let val = self.b;
                        self.bit(val, 5);
                    }
                    0x69 => {
                        // BIT C, 5
                        let val = self.c;
                        self.bit(val, 5);
                    }
                    0x6a => {
                        // BIT D, 5
                        let val = self.d;
                        self.bit(val, 5);
                    }
                    0x6b => {
                        // BIT E, 5
                        let val = self.e;
                        self.bit(val, 5);
                    }
                    0x6c => {
                        // BIT H, 5
                        let val = self.h;
                        self.bit(val, 5);
                    }
                    0x6d => {
                        // BIT L, 5
                        let val = self.l;
                        self.bit(val, 5);
                    }
                    0x6e => {
                        // BIT (HL), 5
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 5);
                    }
                    0x6f => {
                        // BIT A, 5
                        let val = self.a;
                        self.bit(val, 5);
                    }
                    0x70 => {
                        // BIT B, 6
                        let val = self.b;
                        self.bit(val, 6);
                    }
                    0x71 => {
                        // BIT C, 6
                        let val = self.c;
                        self.bit(val, 6);
                    }
                    0x72 => {
                        // BIT D, 6
                        let val = self.d;
                        self.bit(val, 6);
                    }
                    0x73 => {
                        // BIT E, 6
                        let val = self.e;
                        self.bit(val, 6);
                    }
                    0x74 => {
                        // BIT H, 6
                        let val = self.h;
                        self.bit(val, 6);
                    }
                    0x75 => {
                        // BIT L, 6
                        let val = self.l;
                        self.bit(val, 6);
                    }
                    0x76 => {
                        // BIT (HL), 6
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 6);
                    }
                    0x77 => {
                        // BIT A, 6
                        let val = self.a;
                        self.bit(val, 6);
                    }
                    0x78 => {
                        // BIT B, 7
                        let val = self.b;
                        self.bit(val, 7);
                    }
                    0x79 => {
                        // BIT C, 7
                        let val = self.c;
                        self.bit(val, 7);
                    }
                    0x7a => {
                        // BIT D, 7
                        let val = self.d;
                        self.bit(val, 7);
                    }
                    0x7b => {
                        // BIT E, 7
                        let val = self.e;
                        self.bit(val, 7);
                    }
                    0x7c => {
                        // BIT H, 7
                        let val = self.h;
                        self.bit(val, 7);
                    }
                    0x7d => {
                        // BIT L, 7
                        let val = self.l;
                        self.bit(val, 7);
                    }
                    0x7e => {
                        // BIT (HL), 7
                        let val = interconnect.read_byte(self.hl());
                        self.bit(val, 7);
                    }
                    0x7f => {
                        // BIT A, 7
                        let val = self.a;
                        self.bit(val, 7);
                    }
                    0x80 => {
                        // RES B, 0
                        let val = self.b;
                        self.b = self.res(val, 0);
                    }
                    0x81 => {
                        // RES C, 0
                        let val = self.c;
                        self.c = self.res(val, 0);
                    }
                    0x82 => {
                        // RES D, 0
                        let val = self.d;
                        self.d = self.res(val, 0);
                    }
                    0x83 => {
                        // RES E, 0
                        let val = self.e;
                        self.e = self.res(val, 0);
                    }
                    0x84 => {
                        // RES H, 0
                        let val = self.h;
                        self.h = self.res(val, 0);
                    }
                    0x85 => {
                        // RES L, 0
                        let val = self.l;
                        self.l = self.res(val, 0);
                    }
                    0x86 => {
                        // RES (HL), 0
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 0));
                    }
                    0x87 => {
                        // RES A, 0
                        let val = self.a;
                        self.a = self.res(val, 0);
                    }
                    0x88 => {
                        // RES B, 1
                        let val = self.b;
                        self.b = self.res(val, 1);
                    }
                    0x89 => {
                        // RES C, 1
                        let val = self.c;
                        self.c = self.res(val, 1);
                    }
                    0x8a => {
                        // RES D, 1
                        let val = self.d;
                        self.d = self.res(val, 1);
                    }
                    0x8b => {
                        // RES E, 1
                        let val = self.e;
                        self.e = self.res(val, 1);
                    }
                    0x8c => {
                        // RES H, 1
                        let val = self.h;
                        self.h = self.res(val, 1);
                    }
                    0x8d => {
                        // RES L, 1
                        let val = self.l;
                        self.l = self.res(val, 1);
                    }
                    0x8e => {
                        // RES (HL), 1
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 1));
                    }
                    0x8f => {
                        // RES A, 1
                        let val = self.a;
                        self.a = self.res(val, 1);
                    }
                    0x90 => {
                        // RES B, 2
                        let val = self.b;
                        self.b = self.res(val, 2);
                    }
                    0x91 => {
                        // RES C, 2
                        let val = self.c;
                        self.c = self.res(val, 2);
                    }
                    0x92 => {
                        // RES D, 2
                        let val = self.d;
                        self.d = self.res(val, 2);
                    }
                    0x93 => {
                        // RES E, 2
                        let val = self.e;
                        self.e = self.res(val, 2);
                    }
                    0x94 => {
                        // RES H, 2
                        let val = self.h;
                        self.h = self.res(val, 2);
                    }
                    0x95 => {
                        // RES L, 2
                        let val = self.l;
                        self.l = self.res(val, 2);
                    }
                    0x96 => {
                        // RES (HL), 2
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 2));
                    }
                    0x97 => {
                        // RES A, 2
                        let val = self.a;
                        self.a = self.res(val, 2);
                    }
                    0x98 => {
                        // RES B, 3
                        let val = self.b;
                        self.b = self.res(val, 3);
                    }
                    0x99 => {
                        // RES C, 3
                        let val = self.c;
                        self.c = self.res(val, 3);
                    }
                    0x9a => {
                        // RES D, 3
                        let val = self.d;
                        self.d = self.res(val, 3);
                    }
                    0x9b => {
                        // RES E, 3
                        let val = self.e;
                        self.e = self.res(val, 3);
                    }
                    0x9c => {
                        // RES H, 3
                        let val = self.h;
                        self.h = self.res(val, 3);
                    }
                    0x9d => {
                        // RES L, 3
                        let val = self.l;
                        self.l = self.res(val, 3);
                    }
                    0x9e => {
                        // RES (HL), 3
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 3));
                    }
                    0x9f => {
                        // RES A, 3
                        let val = self.a;
                        self.a = self.res(val, 3);
                    }
                    0xa0 => {
                        // RES B, 4
                        let val = self.b;
                        self.b = self.res(val, 4);
                    }
                    0xa1 => {
                        // RES C, 4
                        let val = self.c;
                        self.c = self.res(val, 4);
                    }
                    0xa2 => {
                        // RES D, 4
                        let val = self.d;
                        self.d = self.res(val, 4);
                    }
                    0xa3 => {
                        // RES E, 4
                        let val = self.e;
                        self.e = self.res(val, 4);
                    }
                    0xa4 => {
                        // RES H, 4
                        let val = self.h;
                        self.h = self.res(val, 4);
                    }
                    0xa5 => {
                        // RES L, 4
                        let val = self.l;
                        self.l = self.res(val, 4);
                    }
                    0xa6 => {
                        // RES (HL), 4
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 4));
                    }
                    0xa7 => {
                        // RES A, 4
                        let val = self.a;
                        self.a = self.res(val, 4);
                    }
                    0xa8 => {
                        // RES B, 5
                        let val = self.b;
                        self.b = self.res(val, 5);
                    }
                    0xa9 => {
                        // RES C, 5
                        let val = self.c;
                        self.c = self.res(val, 5);
                    }
                    0xaa => {
                        // RES D, 5
                        let val = self.d;
                        self.d = self.res(val, 5);
                    }
                    0xab => {
                        // RES E, 5
                        let val = self.e;
                        self.e = self.res(val, 5);
                    }
                    0xac => {
                        // RES H, 5
                        let val = self.h;
                        self.h = self.res(val, 5);
                    }
                    0xad => {
                        // RES L, 5
                        let val = self.l;
                        self.l = self.res(val, 5);
                    }
                    0xae => {
                        // RES (HL), 5
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 5));
                    }
                    0xaf => {
                        // RES A, 5
                        let val = self.a;
                        self.a = self.res(val, 5);
                    }
                    0xb0 => {
                        // RES B, 6
                        let val = self.b;
                        self.b = self.res(val, 6);
                    }
                    0xb1 => {
                        // RES C, 6
                        let val = self.c;
                        self.c = self.res(val, 6);
                    }
                    0xb2 => {
                        // RES D, 6
                        let val = self.d;
                        self.d = self.res(val, 6);
                    }
                    0xb3 => {
                        // RES E, 6
                        let val = self.e;
                        self.e = self.res(val, 6);
                    }
                    0xb4 => {
                        // RES H, 6
                        let val = self.h;
                        self.h = self.res(val, 6);
                    }
                    0xb5 => {
                        // RES L, 6
                        let val = self.l;
                        self.l = self.res(val, 6);
                    }
                    0xb6 => {
                        // RES (HL), 6
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 6));
                    }
                    0xb7 => {
                        // RES A, 6
                        let val = self.a;
                        self.a = self.res(val, 6);
                    }
                    0xb8 => {
                        // RES B, 7
                        let val = self.b;
                        self.b = self.res(val, 7);
                    }
                    0xb9 => {
                        // RES C, 7
                        let val = self.c;
                        self.c = self.res(val, 7);
                    }
                    0xba => {
                        // RES D, 7
                        let val = self.d;
                        self.d = self.res(val, 7);
                    }
                    0xbb => {
                        // RES E, 7
                        let val = self.e;
                        self.e = self.res(val, 7);
                    }
                    0xbc => {
                        // RES H, 7
                        let val = self.h;
                        self.h = self.res(val, 7);
                    }
                    0xbd => {
                        // RES L, 7
                        let val = self.l;
                        self.l = self.res(val, 7);
                    }
                    0xbe => {
                        // RES (HL), 7
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.res(val, 7));
                    }
                    0xbf => {
                        // RES A, 7
                        let val = self.a;
                        self.a = self.res(val, 7);
                    }
                    0xc0 => {
                        // SET B, 0
                        let val = self.b;
                        self.b = self.set(val, 0);
                    }
                    0xc1 => {
                        // SET C, 0
                        let val = self.c;
                        self.c = self.set(val, 0);
                    }
                    0xc2 => {
                        // SET D, 0
                        let val = self.d;
                        self.d = self.set(val, 0);
                    }
                    0xc3 => {
                        // SET E, 0
                        let val = self.e;
                        self.e = self.set(val, 0);
                    }
                    0xc4 => {
                        // SET H, 0
                        let val = self.h;
                        self.h = self.set(val, 0);
                    }
                    0xc5 => {
                        // SET L, 0
                        let val = self.l;
                        self.l = self.set(val, 0);
                    }
                    0xc6 => {
                        // SET (HL), 0
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 0));
                    }
                    0xc7 => {
                        // SET A, 0
                        let val = self.a;
                        self.a = self.set(val, 0);
                    }
                    0xc8 => {
                        // SET B, 1
                        let val = self.b;
                        self.b = self.set(val, 1);
                    }
                    0xc9 => {
                        // SET C, 1
                        let val = self.c;
                        self.c = self.set(val, 1);
                    }
                    0xca => {
                        // SET D, 1
                        let val = self.d;
                        self.d = self.set(val, 1);
                    }
                    0xcb => {
                        // SET E, 1
                        let val = self.e;
                        self.e = self.set(val, 1);
                    }
                    0xcc => {
                        // SET H, 1
                        let val = self.h;
                        self.h = self.set(val, 1);
                    }
                    0xcd => {
                        // SET L, 1
                        let val = self.l;
                        self.l = self.set(val, 1);
                    }
                    0xce => {
                        // SET (HL), 1
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 1));
                    }
                    0xcf => {
                        // SET A, 1
                        let val = self.a;
                        self.a = self.set(val, 1);
                    }
                    0xd0 => {
                        // SET B, 2
                        let val = self.b;
                        self.b = self.set(val, 2);
                    }
                    0xd1 => {
                        // SET C, 2
                        let val = self.c;
                        self.c = self.set(val, 2);
                    }
                    0xd2 => {
                        // SET D, 2
                        let val = self.d;
                        self.d = self.set(val, 2);
                    }
                    0xd3 => {
                        // SET E, 2
                        let val = self.e;
                        self.e = self.set(val, 2);
                    }
                    0xd4 => {
                        // SET H, 2
                        let val = self.h;
                        self.h = self.set(val, 2);
                    }
                    0xd5 => {
                        // SET L, 2
                        let val = self.l;
                        self.l = self.set(val, 2);
                    }
                    0xd6 => {
                        // SET (HL), 2
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 2));
                    }
                    0xd7 => {
                        // SET A, 2
                        let val = self.a;
                        self.a = self.set(val, 2);
                    }
                    0xd8 => {
                        // SET B, 3
                        let val = self.b;
                        self.b = self.set(val, 3);
                    }
                    0xd9 => {
                        // SET C, 3
                        let val = self.c;
                        self.c = self.set(val, 3);
                    }
                    0xda => {
                        // SET D, 3
                        let val = self.d;
                        self.d = self.set(val, 3);
                    }
                    0xdb => {
                        // SET E, 3
                        let val = self.e;
                        self.e = self.set(val, 3);
                    }
                    0xdc => {
                        // SET H, 3
                        let val = self.h;
                        self.h = self.set(val, 3);
                    }
                    0xdd => {
                        // SET L, 3
                        let val = self.l;
                        self.l = self.set(val, 3);
                    }
                    0xde => {
                        // SET (HL), 3
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 3));
                    }
                    0xdf => {
                        // SET A, 3
                        let val = self.a;
                        self.a = self.set(val, 3);
                    }
                    0xe0 => {
                        // SET B, 4
                        let val = self.b;
                        self.b = self.set(val, 4);
                    }
                    0xe1 => {
                        // SET C, 4
                        let val = self.c;
                        self.c = self.set(val, 4);
                    }
                    0xe2 => {
                        // SET D, 4
                        let val = self.d;
                        self.d = self.set(val, 4);
                    }
                    0xe3 => {
                        // SET E, 4
                        let val = self.e;
                        self.e = self.set(val, 4);
                    }
                    0xe4 => {
                        // SET H, 4
                        let val = self.h;
                        self.h = self.set(val, 4);
                    }
                    0xe5 => {
                        // SET L, 4
                        let val = self.l;
                        self.l = self.set(val, 4);
                    }
                    0xe6 => {
                        // SET (HL), 4
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 4));
                    }
                    0xe7 => {
                        // SET A, 4
                        let val = self.a;
                        self.a = self.set(val, 4);
                    }
                    0xe8 => {
                        // SET B, 5
                        let val = self.b;
                        self.b = self.set(val, 5);
                    }
                    0xe9 => {
                        // SET C, 5
                        let val = self.c;
                        self.c = self.set(val, 5);
                    }
                    0xea => {
                        // SET D, 5
                        let val = self.d;
                        self.d = self.set(val, 5);
                    }
                    0xeb => {
                        // SET E, 5
                        let val = self.e;
                        self.e = self.set(val, 5);
                    }
                    0xec => {
                        // SET H, 5
                        let val = self.h;
                        self.h = self.set(val, 5);
                    }
                    0xed => {
                        // SET L, 5
                        let val = self.l;
                        self.l = self.set(val, 5);
                    }
                    0xee => {
                        // SET (HL), 5
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 5));
                    }
                    0xef => {
                        // SET A, 5
                        let val = self.a;
                        self.a = self.set(val, 5);
                    }
                    0xf0 => {
                        // SET B, 6
                        let val = self.b;
                        self.b = self.set(val, 6);
                    }
                    0xf1 => {
                        // SET C, 6
                        let val = self.c;
                        self.c = self.set(val, 6);
                    }
                    0xf2 => {
                        // SET D, 6
                        let val = self.d;
                        self.d = self.set(val, 6);
                    }
                    0xf3 => {
                        // SET E, 6
                        let val = self.e;
                        self.e = self.set(val, 6);
                    }
                    0xf4 => {
                        // SET H, 6
                        let val = self.h;
                        self.h = self.set(val, 6);
                    }
                    0xf5 => {
                        // SET L, 6
                        let val = self.l;
                        self.l = self.set(val, 6);
                    }
                    0xf6 => {
                        // SET (HL), 6
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 6));
                    }
                    0xf7 => {
                        // SET A, 6
                        let val = self.a;
                        self.a = self.set(val, 6);
                    }
                    0xf8 => {
                        // SET B, 7
                        let val = self.b;
                        self.b = self.set(val, 7);
                    }
                    0xf9 => {
                        // SET C, 7
                        let val = self.c;
                        self.c = self.set(val, 7);
                    }
                    0xfa => {
                        // SET D, 7
                        let val = self.d;
                        self.d = self.set(val, 7);
                    }
                    0xfb => {
                        // SET E, 7
                        let val = self.e;
                        self.e = self.set(val, 7);
                    }
                    0xfc => {
                        // SET H, 7
                        let val = self.h;
                        self.h = self.set(val, 7);
                    }
                    0xfd => {
                        // SET L, 7
                        let val = self.l;
                        self.l = self.set(val, 7);
                    }
                    0xfe => {
                        // SET (HL), 7
                        let val = interconnect.read_byte(self.hl());
                        interconnect.write_byte(self.hl(), self.set(val, 7));
                    }
                    0xff => {
                        // SET A, 7
                        let val = self.a;
                        self.a = self.set(val, 7);
                    }
                    _ => panic!("Unrecognized extended instruction {:02x}", sub_instr),
                }
                cycle_count += CB_CYCLE_COUNTS[sub_instr as usize];
            }
            0xcc => {
                // CALL Z, nn - Call function at nn if zero flag is set
                let addr = self.read_pc_halfword(interconnect);

                if self.f.z {
                    self.call(interconnect, addr);
                    cycle_count += 12;
                }
            }
            0xcd => {
                // CALL nn - Call function at nn
                let addr = self.read_pc_halfword(interconnect);

                self.call(interconnect, addr);
            }
            0xce => {
                // ADDC A, n
                let val = self.read_pc_byte(interconnect);
                let carry = self.f.c;

                self.a = self.addc(val, carry);
            }
            0xcf => {
                // RST 08
                self.call(interconnect, 0x0008);
            }
            0xd0 => {
                // RET NC
                if !self.f.c {
                    self.ret(interconnect);
                    cycle_count += 12;
                }
            }
            0xd1 => {
                // POP DE
                let e = self.pop_byte(interconnect);
                let d = self.pop_byte(interconnect);

                self.d = d;
                self.e = e;
            }
            0xd2 => {
                // JP NC, nn - Jump to address nn if NC
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                if !self.f.c {
                    self.pc = ((msb as u16) << 8) | lsb as u16;
                    cycle_count += 4;
                }
            }
            0xd4 => {
                // CALL NC, nn - Call function at nn if carry flag is not set
                let addr = self.read_pc_halfword(interconnect);

                if !self.f.c {
                    self.call(interconnect, addr);
                    cycle_count += 12;
                }
            }
            0xd5 => {
                // PUSH DE
                let halfword = self.de();
                self.push_halfword(interconnect, halfword);
            }
            0xd6 => {
                // SUB A, n
                let n = self.read_pc_byte(interconnect);
                self.a = self.subc(n, false);
            }
            0xd7 => {
                // RST 10
                self.call(interconnect, 0x0010);
            }
            0xd8 => {
                // RET C - return if the C flag is set
                if self.f.c {
                    self.ret(interconnect);
                    cycle_count += 12;
                }
            }
            0xd9 => {
                // RETI - return and enable interrupts
                self.ret(interconnect);
                self.interrupts_enabled = true;
            }
            0xda => {
                // JP C, nn - Jump to address nn if C
                let lsb = self.read_pc_byte(interconnect);
                let msb = self.read_pc_byte(interconnect);

                if self.f.c {
                    self.pc = ((msb as u16) << 8) | lsb as u16;
                    cycle_count += 4;
                }
            }
            0xdc => {
                // CALL C, nn - Call function at nn if carry flag is set
                let addr = self.read_pc_halfword(interconnect);

                if self.f.c {
                    self.call(interconnect, addr);
                    cycle_count += 12;
                }
            }
            0xde => {
                // SUBC A, n
                let n = self.read_pc_byte(interconnect);
                let carry = self.f.c;
                self.a = self.subc(n, carry);
            }
            0xdf => {
                // RST 18
                self.call(interconnect, 0x0018);
            }
            0xe0 => {
                // LDH (n), A - Store A in memory 0xff00+n
                let n = self.read_pc_byte(interconnect);
                let addr = 0xff00 + (n as u16);

                interconnect.write_byte(addr, self.a);
            }
            0xe1 => {
                // POP HL
                let l = self.pop_byte(interconnect);
                let h = self.pop_byte(interconnect);

                self.h = h;
                self.l = l;
            }
            0xe2 => {
                // LD (C), A
                let addr = 0xff00 + (self.c as u16);
                interconnect.write_byte(addr, self.a);
            }
            0xe5 => {
                // PUSH HL
                let h = self.h;
                let l = self.l;

                self.push_byte(interconnect, h);
                self.push_byte(interconnect, l);
            }
            0xe6 => {
                let val = self.read_pc_byte(interconnect);
                self.a = self.and(val);
            }
            0xe7 => {
                // RST 20
                self.call(interconnect, 0x0020);
            }
            0xe8 => {
                // ADD SP, n - Add 8 bit immediate to SP
                let n = self.read_pc_byte(interconnect) as i8 as u16;
                let sp = self.sp;

                let res = sp.wrapping_add(n);

                self.sp = res;
                self.f.z = false;
                self.f.n = false;
                self.f.h = ((sp & 0x0f) + (n & 0xf)) > 0xf;
                self.f.c = ((sp & 0xff) + (n & 0xff)) > 0xff;
            }
            0xe9 => {
                // JP (HL) - jump to the address contained in HL
                self.pc = self.hl();
            }
            0xea => {
                // LD nn, A - Store A to immediate address
                let addr = self.read_pc_halfword(interconnect);
                interconnect.write_byte(addr, self.a);
            }
            0xee => {
                let val = self.read_pc_byte(interconnect);
                self.a = self.xor(val);
            }
            0xef => {
                // RST 28
                self.call(interconnect, 0x0028);
            }
            0xf0 => {
                let n = self.read_pc_byte(interconnect);
                let addr = 0xff00 + (n as u16);

                self.a = interconnect.read_byte(addr);
            }
            0xf1 => {
                // POP AF
                let f = self.pop_byte(interconnect);
                let a = self.pop_byte(interconnect);

                self.a = a;
                self.f = f.into();
            }
            0xf2 => {
                let addr = 0xff00 + (self.c as u16);
                self.a = interconnect.read_byte(addr);
            }
            0xf3 => {
                // DI -Disable interrupts after the next instruction is executed
                self.instructions_to_di = 1;
            }
            0xf5 => {
                // PUSH AF
                let a = self.a;
                let f = self.f;

                self.push_byte(interconnect, a);
                self.push_byte(interconnect, f.into());
            }
            0xf6 => {
                let val = self.read_pc_byte(interconnect);
                self.a = self.or(val);
            }
            0xf7 => {
                // RST 30
                self.call(interconnect, 0x0030);
            }
            0xf8 => {
                // LD HL, SP+n
                let n = self.read_pc_byte(interconnect) as i8 as u16;
                let addr = self.sp.wrapping_add(n);

                self.f.z = false;
                self.f.n = false;
                self.f.h = ((self.sp & 0x0f) + (n & 0xf)) > 0xf;
                self.f.c = ((self.sp & 0xff) + (n & 0xff)) > 0xff;

                self.h = (addr >> 8) as u8;
                self.l = (addr & 0xff) as u8;
            }
            0xf9 => self.sp = self.hl(), // LD SP, HL
            0xfa => {
                let addr = self.read_pc_halfword(interconnect);
                self.a = interconnect.read_byte(addr);
            }
            0xfb => {
                // EI
                self.interrupts_enabled = true;
            }
            0xfe => {
                let val = self.read_pc_byte(interconnect);
                self.subc(val, false);
            }
            0xff => {
                // RST 38
                self.call(interconnect, 0x0038);
            }
            _ => panic!("Unrecognized instruction {:02x} at {:04x}", instr, old_pc),
        }

        if self.instructions_to_di > 0 {
            self.instructions_to_di -= 1;
            if self.instructions_to_di == 0 {
                self.disable_interrupts();
            }
        }

        self.total_cycles += cycle_count as u32;
        cycle_count
    }

    fn read_pc_byte(&mut self, interconnect: &Interconnect) -> u8 {
        let val = interconnect.read_byte(self.pc);
        if self.halted == -1 {
            self.halted = 0;
        } else {
            self.pc += 1;
        }
        val
    }

    fn read_pc_halfword(&mut self, interconnect: &Interconnect) -> u16 {
        let lsb = self.read_pc_byte(interconnect);
        let msb = self.read_pc_byte(interconnect);

        ((msb as u16) << 8) | (lsb as u16)
    }

    fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    fn handle_interrupt(&mut self, interconnect: &mut Interconnect, int_f: u8, int_e: u8) {
        let interrupt_vector = int_f & int_e;
        let interrupt = interrupt_vector.trailing_zeros();

        let addr = match interrupt {
            0 => 0x0040,
            1 => 0x0048,
            2 => 0x0050,
            3 => 0x0058,
            4 => 0x0060,
            _ => unreachable!(),
        };

        interconnect.write_byte(0xff0f, int_f & !(1 << interrupt));

        self.call(interconnect, addr);
        self.halted = 0;
    }

    fn push_halfword(&mut self, interconnect: &mut Interconnect, addr: u16) {
        self.sp -= 2;
        interconnect.write_halfword(self.sp, addr);
    }

    fn push_byte(&mut self, interconnect: &mut Interconnect, val: u8) {
        self.sp -= 1;
        interconnect.write_byte(self.sp, val);
    }

    fn pop_halfword(&mut self, interconnect: &mut Interconnect) -> u16 {
        let ret = interconnect.read_halfword(self.sp);
        self.sp += 2;
        ret
    }

    fn pop_byte(&mut self, interconnect: &mut Interconnect) -> u8 {
        let val = interconnect.read_byte(self.sp);
        self.sp += 1;
        val
    }

    fn call(&mut self, interconnect: &mut Interconnect, addr: u16) {
        let pc = self.pc;
        self.push_halfword(interconnect, pc);
        self.pc = addr;
    }

    fn ret(&mut self, interconnect: &mut Interconnect) {
        let addr = self.pop_halfword(interconnect);
        self.pc = addr;
    }

    fn addc(&mut self, val: u8, carry: bool) -> u8 {
        let carry = if carry { 1 } else { 0 };
        let (tmp, overflow) = self.a.overflowing_add(val);
        let (r, overflow_c) = tmp.overflowing_add(carry);

        self.f.z = r == 0;
        self.f.n = false;
        self.f.h = ((self.a & 0xf) + (val & 0xf) + carry) > 0xf;
        self.f.c = overflow || overflow_c;

        r
    }

    fn add16(&mut self, lhs: u16, rhs: u16) -> u16 {
        let (ret, overflow) = lhs.overflowing_add(rhs);

        self.f.n = false;
        self.f.h = ((lhs & 0x0fff) + (rhs & 0x0fff)) > 0x0fff;
        self.f.c = overflow;

        ret
    }

    fn subc(&mut self, val: u8, carry: bool) -> u8 {
        let carry = if carry { 1 } else { 0 };
        let (tmp, underflow) = self.a.overflowing_sub(val);
        let (r, underflow_c) = tmp.overflowing_sub(carry);

        self.f.z = r == 0;
        self.f.n = true;
        self.f.h = ((val & 0xf) + carry) > (self.a & 0xf);
        self.f.c = underflow || underflow_c;

        r
    }

    fn and(&mut self, val: u8) -> u8 {
        let r = self.a & val;

        self.f.z = r == 0;
        self.f.n = false;
        self.f.h = true;
        self.f.c = false;

        r
    }

    fn or(&mut self, val: u8) -> u8 {
        let r = self.a | val;

        self.f.z = r == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = false;

        r
    }

    fn xor(&mut self, val: u8) -> u8 {
        let r = self.a ^ val;

        self.f.z = r == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = false;

        r
    }

    fn inc(&mut self, val: u8) -> u8 {
        let r = val.wrapping_add(1);

        self.f.z = r == 0;
        self.f.n = false;
        self.f.h = (r & 0x0f) == 0;

        r
    }

    fn dec(&mut self, val: u8) -> u8 {
        let r = val.wrapping_sub(1);

        self.f.z = r == 0;
        self.f.n = true;
        self.f.h = (r & 0x0f) == 0x0f;

        r
    }

    fn bit(&mut self, val: u8, bit: u8) {
        self.f.z = (val & (1 << bit)) == 0;
        self.f.n = false;
        self.f.h = true;
    }

    fn set(&mut self, val: u8, bit: u8) -> u8 {
        val | (1 << bit)
    }

    fn res(&mut self, val: u8, bit: u8) -> u8 {
        val & !(1 << bit)
    }

    fn swap(&mut self, val: u8) -> u8 {
        let ret = ((val & 0x0f) << 4) | ((val & 0xf0) >> 4);

        self.f.z = val == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = false;

        ret
    }

    fn rl(&mut self, val: u8) -> u8 {
        let carry = if self.f.c { 1 } else { 0 };
        let ret = (val << 1) | carry;

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = (val & 0x80) != 0;

        ret
    }

    fn rlc(&mut self, val: u8) -> u8 {
        let carry = val >> 7;
        let ret = (val << 1) | carry;

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = carry != 0;

        ret
    }

    fn rr(&mut self, val: u8) -> u8 {
        let carry = if self.f.c { 1 } else { 0 };
        let ret = (val >> 1) | (carry << 7);

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = (val & 0x01) != 0;

        ret
    }

    fn rrc(&mut self, val: u8) -> u8 {
        let carry = val & 0x01;
        let ret = (val >> 1) | (carry << 7);

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = (val & 0x01) != 0;

        ret
    }

    fn srl(&mut self, val: u8) -> u8 {
        let ret = val >> 1;

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = (val & 0x01) != 0;

        ret
    }

    fn sla(&mut self, val: u8) -> u8 {
        let ret = val << 1;

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = (val & 0x80) != 0;

        ret
    }

    fn sra(&mut self, val: u8) -> u8 {
        let ret = (val & 0x80) | (val >> 1);

        self.f.z = ret == 0;
        self.f.n = false;
        self.f.h = false;
        self.f.c = (val & 0x01) != 0;

        ret
    }

    pub fn af(&self) -> u16 {
        let f: u8 = self.f.into();
        ((self.a as u16) << 8) | (f as u16)
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = (val & 0xff) as u8;
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = (val & 0xff) as u8;
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = (val & 0xff) as u8;
    }
}


impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
