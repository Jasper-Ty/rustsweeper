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

impl Render for Board {
    fn render(
        &self, 
        canvas: &mut Canvas<Window>, 
        spritesheet: &Spritesheet) 
    -> Result<(), String> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let rect = Rect::new(x as i32 * SQ_I32, y as i32 * SQ_I32, SQ_U32, SQ_U32);
                match self[(x, y)] {
                    Cell {
                        open: false,
                        flag: true,
                        ..
                    } => {
                            spritesheet.draw(canvas, Sprite::Flag, rect)?; 
                    },
                    Cell {
                        open: false,
                        ..
                    } => {
                            spritesheet.draw(canvas, Sprite::Closed, rect)?; 
                    },
                    Cell {
                        mine: true,
                        ..
                    } => {
                        spritesheet.draw(canvas, Sprite::Mine, rect)?; 
                    },
                    Cell {
                        num: n,
                        ..
                    } => {
                        spritesheet.draw(canvas, Sprite::Num(n), rect)?; 
                    },
                }
            }
        }
        Ok(())
    }
}
