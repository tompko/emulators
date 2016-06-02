use super::sdl2;
use super::memory::Memory;
use super::graphics::Graphics;
use super::input::Input;
use super::cart::Cart;


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
}
