use crate::*;

use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Render {
    fn render(
        &self, 
        canvas: &mut Canvas<Window>, 
        spritesheet: &Spritesheet,
        game_state: &GameState,
        input_state: &InputState,
    ) -> Result<(), String>; 
}

impl Render for Board {
    fn render(
        &self, 
        canvas: &mut Canvas<Window>, 
        spritesheet: &Spritesheet,
        game_state: &GameState,
        input_state: &InputState,
    ) -> Result<(), String> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let rect = rect!(
                    (x*SQ_SIZE) as i32 + BOARD_X,
                    (y*SQ_SIZE) as i32 + BOARD_Y,
                    SQ_SIZE, 
                    SQ_SIZE
                );

                let tentative = false;
                match self[(x, y)] {
                    Cell {
                        open: false,
                        flag: true,
                        mine: false,
                        ..
                    } => {
                        if let GameState::Lose = game_state {
                            spritesheet.draw(canvas, Sprite::MineCross, rect)?; 
                        } else {
                            spritesheet.draw(canvas, Sprite::Flag, rect)?; 
                        } 
                    },
                    Cell {
                        open: false,
                        flag: true,
                        ..
                    } => {
                        spritesheet.draw(canvas, Sprite::Flag, rect)?; 
                    },
                    Cell {
                        open: false,
                        mine: true,
                        ..
                    } => {
                        if let GameState::Lose = game_state {
                            spritesheet.draw(canvas, Sprite::Mine, rect)?; 
                        } else if tentative {
                            spritesheet.draw(canvas, Sprite::Num(0), rect)?; 
                        } else {
                            spritesheet.draw(canvas, Sprite::Closed, rect)?; 
                        }
                    },
                    Cell {
                        open: false,
                        ..
                    } => {
                        if tentative {
                            spritesheet.draw(canvas, Sprite::Num(0), rect)?; 
                        } else {
                            spritesheet.draw(canvas, Sprite::Closed, rect)?; 
                        }
                    },
                    Cell {
                        mine: true,
                        ..
                    } => {
                        spritesheet.draw(canvas, Sprite::MineRed, rect)?; 
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
