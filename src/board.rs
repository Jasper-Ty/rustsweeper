use std::ops::{ Index, IndexMut };

use super::Sprite;
use super::Spritesheet;

use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::rect::Rect;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{ thread_rng };

const SQ_SIZE: usize = 48; 
const SQ_I32: i32 = SQ_SIZE as i32;
const SQ_U32: u32 = SQ_SIZE as u32;

#[derive(Debug, Clone, Copy)]
pub enum Cover {
    Flag,
    Closed,
    Open,
}
pub struct Overlay {
    covers: Vec<Cover>,
    width: usize,
    height: usize,
}
impl Overlay {
    pub fn new(width: usize, height: usize) -> Self {
        let covers: Vec<Cover> = vec![Cover::Closed; width*height];

        Self {
            covers,
            width,
            height,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, spritesheet: &Spritesheet) -> Result<(), String> {
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
impl Index<(usize, usize)> for Overlay {
    type Output = Cover;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.covers[y*self.width + x]
    }
}
impl IndexMut<(usize, usize)> for Overlay {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.covers[y*self.width + x]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Mine,
    Num(u8),
}

pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}
impl Board {
    pub fn new_random(width: usize, height: usize, num_mines: usize) -> Self {
        let cells: Vec<Cell> = vec![Cell::Num(0); width*height];

        let mut board = Self {
            cells,
            width,
            height,
        };

        board.gen_random_mines(num_mines);
        board.gen_numbers();

        board
    }

    pub fn gen_random_mines(&mut self, num_mines: usize) {
        let mut rng = thread_rng();
        for i in 0..num_mines {
            self.cells[i] = Cell::Mine;
        }
        self.cells.shuffle(&mut rng);
    }

    pub fn gen_numbers(&mut self) {
        let neighborhood = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            if let Cell::Num(_) = self[(x, y)] {
                self[(x, y)] = Cell::Num(
                    neighborhood.iter()
                        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                        .filter(|(x, y)| 
                            *x >= 0 && *x < self.width as i32
                            && *y >= 0 && *y < self.height as i32)
                        .map(|(x, y)| match self[(x as usize, y as usize)] {
                            Cell::Mine => 1,
                            _ => 0,
                        })
                        .sum()
                );
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, spritesheet: &Spritesheet) -> Result<(), String> {
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
impl Index<(usize, usize)> for Board {
    type Output = Cell;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.cells[y*self.width + x]
    }
}
impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.cells[y*self.width + x]
    }
}
