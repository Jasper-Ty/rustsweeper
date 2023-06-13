pub mod board;

pub use board::*;

use std::path::Path;

use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::video::Window;

#[derive(Clone, Copy, Debug)]
pub enum Sprite {
    Mine,
    MineRed,
    MineCross,
    Num(u8),
    Counter(u8),
    Closed,
    Flag,
    BtnSmile,
    BtnDown,
    BtnSurprise,
    BtnSunglasses,
    BtnDead,
}

macro_rules! rect {
    ($x: expr, $y: expr, $w: expr, $h: expr) => {
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    };
}

impl Sprite {
    pub fn get_rect(&self) -> Rect {
        match self {
            Sprite::Mine => rect!(85, 51, 16, 16),
            Sprite::MineRed => rect!(102, 51, 16, 16),
            Sprite::MineCross => rect!(119, 51, 16, 16),

            Sprite::Num(0) => rect!(17, 51, 16, 16),
            Sprite::Num(i @ 1..=8) => rect!(17*(i-1), 68, 16, 16),
            Sprite::Counter(i @ 0..=9) => rect!(i*14, 0, 13, 23),

            Sprite::Closed => rect!(0, 51, 16, 16),
            Sprite::Flag => rect!(34, 51, 16, 16),

            Sprite::BtnSmile => rect!(0, 24, 26, 26),
            Sprite::BtnDown => rect!(27, 24, 26, 26),
            Sprite::BtnSurprise => rect!(54, 24, 26, 26),
            Sprite::BtnSunglasses => rect!(81, 24, 26, 26),
            Sprite::BtnDead => rect!(108, 24, 26, 26),
            _ => panic!("Invalid Sprite value?")
        }
    }
}

pub struct Spritesheet<'a>(Texture<'a>);

impl<'a> Spritesheet<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let texture = Surface::load_bmp(Path::new("./spritesheet.bmp"))?
            .as_texture(texture_creator)
            .map_err(|e| e.to_string())?;

        Ok(Self(texture))
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>, sprite: Sprite, draw_rect: Rect) -> Result<(), String> {
        canvas.copy(
            &self.0,
            sprite.get_rect(),
            draw_rect,
        )?;

        Ok(())
    }
}
