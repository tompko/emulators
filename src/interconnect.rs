use super::sdl2;
use super::memory::Memory;
use super::graphics::Graphics;
use super::input::Input;
use super::cart::Cart;
use super::mem_map::*;


pub struct Interconnect {
    pub cpu_ram: Memory,
    pub graphics: Graphics,
    pub input: Input,
    pub cart: Cart,
}

impl Interconnect {
    pub fn new(cart_rom: Box<[u8]>) -> Interconnect {
        let context = sdl2::init().unwrap();

        let cart = Cart::new(cart_rom).unwrap();
         Interconnect{
            cpu_ram: Memory::new(),
            graphics: Graphics::new(&context),
            input: Input::new(&context),
            cart: cart,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match map_addr(addr) {
            Addr::Ram(offset) => self.cpu_ram.read_byte(offset),
            Addr::PrgRom1(offset) => self.cart.read_prg_byte(0, offset),
            Addr::PrgRom2(offset) => self.cart.read_prg_byte(1, offset),
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        match map_addr(addr) {
            Addr::Ram(offset) => self.cpu_ram.read_word(offset),
            Addr::PrgRom1(offset) => self.cart.read_prg_word(0, offset),
            Addr::PrgRom2(offset) => self.cart.read_prg_word(1, offset),
        }
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        match map_addr(addr) {
            Addr::Ram(offset) => self.cpu_ram.write_byte(offset, val),
            _ => panic!("Write at unrecognised physical address: {:x}", addr),
        }
    }
}
