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
    Num(u8),
    Open,
    Closed,
    Flag,
}
impl Sprite {
    pub fn get_rect(&self) -> Rect {
        match self {
            Sprite::Mine => Rect::new(85, 51, 16, 16),
            Sprite::Num(i @ 0..=8) => match i {
                0 => Rect::new(17, 51, 16, 16),
                1..=8 => Rect::new(17*(*i as i32-1), 68, 16, 16),
                _ => Rect::new(0, 0, 10, 10),
            }
            _ => Rect::new(0, 0, 16, 16),
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
