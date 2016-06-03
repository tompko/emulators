use super::sdl2;
use super::memory::Memory;
use super::graphics::Graphics;
use super::input::Input;
use super::cart::Cart;
use super::mem_map::*;


pub struct Interconnect {
    pub mem: Memory,
    pub graphics: Graphics,
    pub input: Input,
    pub cart: Cart,
}

impl Interconnect {
    pub fn new(cart_rom: Box<[u8]>) -> Interconnect {
        let context = sdl2::init().unwrap();

        let cart = Cart::new(cart_rom).unwrap();
        return Interconnect{
            mem: Memory::new(),
            graphics: Graphics::new(&context),
            input: Input::new(&context),
            cart: cart,
        }
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        match map_addr(addr) {
            Addr::PrgRom1(offset) => self.cart.read_prg_rom(0, offset),
            Addr::PrgRom2(offset) => self.cart.read_prg_rom(1, offset),
        }
    }
}
