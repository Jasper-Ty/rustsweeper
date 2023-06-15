use std::ops::{ Index, IndexMut };
use std::default::Default;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{ thread_rng };


#[derive(Debug, Clone, Copy)]
pub enum Cover {
    Flag,
    Closed,
    Open,
}
impl Default for Cover {
    fn default() -> Self { Cover::Closed }
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
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

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