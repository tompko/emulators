use super::sdl2;
use super::sdl2::pixels::Color;
use super::sdl2::render::Renderer;
use super::sdl2::rect::Point;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const SCREEN_SCALE: usize = 10;

pub struct Graphics {
    renderer: Renderer<'static>,
    screen: Vec<u8>
}

impl Graphics {
    pub fn new(context: &sdl2::Sdl) -> Graphics {
        let video = context.video().unwrap();

        let window = video.window(
            "chip8",
            (SCREEN_WIDTH*SCREEN_SCALE) as u32,
            (SCREEN_HEIGHT*SCREEN_SCALE) as u32)
        .build().unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_scale(SCREEN_SCALE as f32, SCREEN_SCALE as f32);

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        return Graphics{
            renderer: renderer,
            screen: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> u8 {
        let mut collision = 0;
        for (i, s) in sprite.iter().enumerate() {
            let dy = (y + i) % SCREEN_HEIGHT;
            for j in 0..8 {
                let dx = (x + j) % SCREEN_WIDTH;
                let index = (dy * SCREEN_WIDTH) + dx;
                let val = (s >> j) & 0x1;

                collision != self.screen[index];
                self.screen[index] ^= val;
            }
        }
        return collision;
    }

    pub fn clear(&mut self) {
        for i in 0..SCREEN_SIZE {
            self.screen[i] = 0;
        }
    }

    pub fn render(&mut self) {
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();

        self.renderer.set_draw_color(Color::RGB(0, 255, 0));
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = (y * SCREEN_WIDTH) + x;
                if self.screen[index] == 1 {
                    self.renderer.draw_point(Point::new(x as i32, y as i32));
                }
            }
        }
        self.renderer.present();
    }
}
