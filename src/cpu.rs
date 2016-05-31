use super::interconnect::Interconnect;
use super::memory::END_RESERVED;
use super::rand;
use super::time::{Duration, SteadyTime};

const INSTRUCTION_SIZE: u16 = 2;

pub struct Cpu {
    v: [u8; 16],
    pc: u16,
    i: u16,
    stack: Vec<u16>,

    delay_timer: u8,
    sound_timer: u8,

    delay_start: SteadyTime,
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
            delay_start: SteadyTime::now(),
        }
    }

    pub fn run(&mut self, interconnect: &mut Interconnect) {
        loop {
            let instr = interconnect.mem.read_word(self.pc);

            self.execute_instruction(instr, interconnect);

            self.handle_timers();

            interconnect.graphics.render();
            interconnect.input.handle_input();

            if interconnect.input.quit {
                break;
            }
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
            0x4 => {
                // 4xkk - SNE Vx, byte
                if self.v[x] != kk {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            0x5 => {
                // 5xy0 - SE Vx, Vy
                if self.v[x] == self.v[y] {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            0x6 => {
                // 6xkk - LD Vx, byte
                self.v[x as usize] = kk;
            }
            0x7 => {
                // 7xkk - ADD Vx, byte
                self.v[x] = self.v[x].wrapping_add(kk as u8);
            }
            0x8 => {
                match n {
                    0x0 => {
                        // 8xy0 - LD Vx, Vy
                        self.v[x] = self.v[y];
                    }
                    0x1 => {
                        // 8xy1 - OR Vx, Vy
                        self.v[x] = self.v[x] | self.v[y];
                    }
                    0x2 => {
                        // 8xy2 - AND Vx, Vy
                        self.v[x] = self.v[x] & self.v[y];
                    }
                    0x3 => {
                        // 8xy3 - XOR Vx, Vy
                        self.v[x] = self.v[x] ^ self.v[y];
                    }
                    0x4 => {
                        // 8xy4 - ADD Vx, Vy
                        let ret = (self.v[x] as u16) + (self.v[y] as u16);
                        if ret > 255 {
                            self.v[0xf] = 1;
                        } else {
                            self.v[0xf] = 0;
                        }
                        self.v[x] = ret as u8;
                    }
                    0x5 => {
                        // 8xy5 - SUB Vx, Vy
                        if self.v[x] > self.v[y] {
                            self.v[0xf] = 1;
                        } else {
                            self.v[0xf] = 0;
                        }
                        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
                    }
                    0x6 => {
                        // 8xy6 - SHR Vx {, Vy}
                        self.v[0xf] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                    }
                    0x7 => {
                        // 8xy7 - SUBN Vx, Vy
                        if self.v[y] > self.v[x] {
                            self.v[0xf] = 1;
                        } else {
                            self.v[0xf] = 0;
                        }
                        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
                    }
                    0xE => {
                        self.v[0xf] = self.v[x] >> 7;
                        self.v[x] <<= 1;
                    }
                    _ => panic!("Unrecognized 8 variant {:x}({:x})", instr, n),
                }
            }
            0x9 => {
                // 9xy0 - SNE Vx, Vy
                if self.v[x] != self.v[y] {
                    self.pc += INSTRUCTION_SIZE;
                }
            }
            0xa => {
                // Annn - LD I, addr
                self.i = nnn;
            }
            0xb => {
                // Bnnn - JP V0, addr
                self.pc = (self.v[0] as u16) + nnn;
                jmp = true;
            }
            0xc => {
                // Cxkk - RND Vx, byte
                self.v[x] = rand::random::<u8>() & kk;
            }
            0xd => {
                // Dxyn - DRW Vx, Vy, nibble
                let mut sprite = vec![0; n as usize];
                let vx = self.v[x] as usize;
                let vy = self.v[y] as usize;

                for i in 0..n {
                    sprite[i] = interconnect.mem.read_byte(self.i + i as u16);
                }

                self.v[0xf] = interconnect.graphics.draw(vx, vy, sprite);
            }
            0xe => {
                match kk {
                    0x9e => {
                        // Ex9E - SKP Vx
                        if interconnect.input.key_pressed(self.v[x]) {
                            self.pc += INSTRUCTION_SIZE;
                        }
                    }
                    0xa1 => {
                        // ExA1 - SKNP Vx
                        if !interconnect.input.key_pressed(self.v[x]) {
                            self.pc += INSTRUCTION_SIZE;
                        }
                    }
                    _ => panic!("Unrecognized e variant {:x}({:x})", instr, kk),
                }
            }
            0xf => {
                match kk {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        self.v[x] = self.delay_timer;
                    }
                    0x0a => {
                        // Fx0A - LD Vx, K
                        match interconnect.input.any_key_pressed() {
                            Some(index) => self.v[x] = index,
                            None => jmp = true,
                        }
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        let val = self.v[x];
                        self.delay_timer = val;
                    }
                    0x18 => {
                        // Fx18 - LD ST, Vx
                        self.sound_timer = self.v[x];
                    }
                    0x1E => {
                        // Fx1E - ADD I, Vx
                        self.i += self.v[x] as u16;
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
                    0x55 => {
                        // Fx55 - LD [I], Vx
                        for i in 0..(x + 1) {
                            interconnect.mem.write_byte(self.i + i as u16, self.v[i]);
                        }
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

    fn handle_timers(&mut self) {
        if self.delay_timer > 0 {
            let now = SteadyTime::now();
            if now - self.delay_start > Duration::milliseconds(16) {
                self.delay_start = now;
                self.delay_timer -= 1;
            }
        }

    }
}
