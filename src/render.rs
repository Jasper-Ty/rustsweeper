use crate::*;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Render {
    fn render(
        &self, 
        canvas: &mut Canvas<Window>, 
        spritesheet: &Spritesheet) 
    -> Result<(), String>; 
}

impl Render for Overlay {
    fn render(
        &self, 
        canvas: &mut Canvas<Window>, 
        spritesheet: &Spritesheet) 
    -> Result<(), String> {
        for y in 0..16 {
            for x in 0..30 {
                let rect = Rect::new(x as i32 * SQ_I32, y as i32 * SQ_I32, SQ_U32, SQ_U32);
                match self[(x, y)] {
                    Cover::Flag => {
                        spritesheet.draw(canvas, Sprite::Flag, rect)?; 
                    },
                    Cover::Closed => {
                        spritesheet.draw(canvas, Sprite::Closed, rect)?;
                    },
                    Cover::Open => {},
                }
            }
        }
        Ok(())
    }
}

impl Render for Board {
    fn render(
        &self, 
        canvas: &mut Canvas<Window>, 
        spritesheet: &Spritesheet) 
    -> Result<(), String> {
        for y in 0..16 {
            for x in 0..30 {
                let rect = Rect::new(x as i32 * SQ_I32, y as i32 * SQ_I32, SQ_U32, SQ_U32);
                match self[(x, y)] {
                    Cell::Mine => {
                        spritesheet.draw(canvas, Sprite::Mine, rect)?; 
                    },
                    Cell::Num(n) => {
                        spritesheet.draw(canvas, Sprite::Num(n), rect)?;
                    },
                }
            }
        }
        Ok(())
    }
}
