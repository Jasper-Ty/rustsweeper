use std::ops::{ Index, IndexMut };
use itertools::Itertools;

use sdl2;
use rand::seq::SliceRandom;
use rand::{ thread_rng, Rng };

enum PlayState {
    IDLE,
    PLAY,
    WIN,
    GAMEOVER,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Mine,
    Num(u8),
}

struct Board {
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

    fn gen_random_mines(&mut self, num_mines: usize) {
        let mut rng = thread_rng();
        for i in 0..num_mines {
            self.cells[i] = Cell::Mine;
        }
        self.cells.shuffle(&mut rng);
    }

    fn gen_numbers(&mut self) {
        let neighborhood = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1),
        ];
        for (x, y) in (0..self.width).cartesian_product(0..self.height) {
            if let Cell::Num(_) = self[(x, y)] {
                let n: u8 = neighborhood.iter()
                    .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                    .filter(|(x, y)| 
                        *x > 0 && *x < self.width as i32
                        && *y > 0 && *y < self.height as i32)
                    .map(|(x, y)| match self[(x as usize, y as usize)] {
                        Cell::Mine => 1,
                        _ => 0,
                    })
                    .sum();
                self[(x, y)] = Cell::Num(n);
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

fn main() {
    let state = PlayState::IDLE;

    let board = Board::new_random(30, 16, 99);
    println!("{:?}", board.cells);
}
