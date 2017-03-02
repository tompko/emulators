use super::sdl2;
use super::sdl2::pixels::Color;
use super::sdl2::render::Renderer;
use super::sdl2::rect::Point;


pub struct Graphics {
}

impl Graphics {
    pub fn new(context: &sdl2::Sdl) -> Graphics {
        Graphics{}
    }
}
