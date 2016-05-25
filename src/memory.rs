use super::byteorder::{BigEndian, ByteOrder};

pub const RAM_BYTES: usize = 4*1024;
pub const END_RESERVED: usize = 0x200;

const DIGIT_SIZE: usize = 5;
const DIGIT_COUNT: usize = 16;

static DIGIT_SPRITES: &'static [u8] =&[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Memory {
    ram: Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory{
            ram: vec![0; RAM_BYTES]
        };
        mem.load_fonts();
        return mem;
    }

    fn load_fonts(&mut self) {
        for i in 0..DIGIT_COUNT {
            for j in 0..DIGIT_SIZE {
                let offset = i * DIGIT_SIZE + j;
                self.ram[offset] = DIGIT_SPRITES[offset];
            }
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for i in 0..rom.len() {
            self.ram[i + END_RESERVED] = rom[i];
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        return BigEndian::read_u16(&self.ram[addr as usize..]);
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        return self.ram[addr as usize];
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.ram[addr as usize] = val;
    }
}
