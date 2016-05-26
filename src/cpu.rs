use super::interconnect::Interconnect;
use super::memory::END_RESERVED;

const INSTRUCTION_SIZE: u16 = 2;

pub struct Cpu {
    v: [u8; 16],
    pc: u16,
    i: u16,
    stack: Vec<u16>,

    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu{
            v: [0; 16],
            pc: END_RESERVED as u16,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn run(&mut self, interconnect: &mut Interconnect) {
        loop {
            let instr = interconnect.mem.read_word(self.pc);

            self.execute_instruction(instr, interconnect);
        }
    }

    pub fn execute_instruction(&mut self, instr: u16, interconnect: &mut Interconnect) {
        let opcode = (instr >> 12) as u8;
        let nnn = instr & 0xfff;
        let x = ((instr >> 8) & 0xf) as usize;
        let y = ((instr >> 4) & 0xf) as usize;
        let n = (instr & 0xf) as usize;
        let kk = (instr & 0xff) as u8;
        let mut jmp = false;

        match opcode {
            0x0 => {
                match kk {
                    0xe0 => {
                        // 00E0 - CLS
                        interconnect.graphics.clear();
                    }
                    0xee => {
                        // 00EE - RET
                        let ret = self.stack.pop().expect("ret without func call");
                        self.pc = ret;
                    }
                    _ => {
                        // 0nnn - SYS addr
                        // This instruction is only used on the old computers on which Chip-8 was
                        // originally implemented. It is ignored by modern interpreters.
                    }
                }
            }
            0x1 => {
                // 1nnn - JP addr
                self.pc = nnn;
                jmp = true;
            }
            0x2 => {
                // 2nnn - CALL addr
                self.stack.push(self.pc);
                self.pc = nnn;
                jmp = true;
            }
            0x3 => {
                // 3xkk - SE Vx, byte
                if self.v[x] == kk {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            0x6 => {
                // 6xkk - LD Vx, byte
                self.v[x as usize] = kk as u8;
            }
            0x7 => {
                // 7xkk - ADD Vx, byte
                self.v[x] = self.v[x].wrapping_add(kk as u8);
            }
            0xa => {
                // Annn - LD I, addr
                self.i = nnn;
            }
            0xd => {
                // Dxyn - DRW Vx, Vy, nibble
                let mut sprite = vec![0; n as usize];

                for i in 0..n {
                    sprite[i] = interconnect.mem.read_byte(self.i + i as u16);
                }

                self.v[0xf] = interconnect.graphics.draw(x, y, sprite);
            }
            0xf => {
                match kk {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        self.v[x] = self.delay_timer;
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        let val = self.v[x];
                        self.delay_timer = val;
                    }
                    0x29 => {
                        // Fx29 - LD F, Vx
                        let val = self.v[x];
                        self.i = interconnect.mem.get_digit_sprite(val);
                    }
                    0x33 => {
                        // Fx33 - LD B, Vx
                        let val = self.v[x];
                        interconnect.mem.write_byte(self.i, val / 100);
                        interconnect.mem.write_byte(self.i + 1, (val % 100) / 10);
                        interconnect.mem.write_byte(self.i + 2, val % 10);
                    }
                    0x65 => {
                        // Fx65 - LD Vx, [I]
                        for i in 0..(x+1) {
                            self.v[i] = interconnect.mem.read_byte(self.i + i as u16);
                        }
                    }
                    _ => panic!("Unrecognized f variant {:x}({:x})", instr, kk),
                }
            }

            _ => panic!("Unrecognized instruction 0x{:x} ({:x})", instr, opcode),
        }

        if !jmp {
            self.pc += INSTRUCTION_SIZE;
        }
    }
}
