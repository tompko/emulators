use super::sdl2;
use super::sdl2::pixels::Color;
use super::sdl2::render::Renderer;

pub struct Graphics {
    renderer: Renderer<'static>
}

impl Graphics {
    pub fn new(context: &sdl2::Sdl) -> Graphics {
        let video = context.video().unwrap();

        let window = video.window("chip8", 640, 320).build().unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_scale(10.0, 10.0);

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        return Graphics{
            renderer: renderer,
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> u8 {
        return 0;
    }
}
