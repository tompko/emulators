use super::interconnect::Interconnect;
use super::memory::END_RESERVED;

const INSTRUCTION_SIZE: u16 = 2;

pub struct Cpu {
    v: [u8; 16],
    pc: u16,
    i: u16

}

impl Cpu {
    pub fn new() -> Cpu {
        return Cpu{
            v: [0; 16],
            pc: END_RESERVED as u16,
            i: 0,
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
        let kk = instr & 0xff;

        match opcode {

            0x6 => self.v[x as usize] = kk as u8,
            0xa => self.i = nnn,
            0xd => {
                let mut sprite = vec![0; n as usize];

                for i in 0..n {
                    sprite[i] = interconnect.mem.read_byte(self.i + i as u16);
                }

                self.v[0xf] = interconnect.graphics.draw(x, y, sprite);
            }

            _ => panic!("Unrecognized instruction 0x{:x} ({:x})", instr, opcode),
        }

        self.pc += INSTRUCTION_SIZE;
    }
}
