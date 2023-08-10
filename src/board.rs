use std::ops::{ Index, IndexMut };
use std::default::Default;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub const BOARD_X: i32 = 0;
pub const BOARD_Y: i32 = 26;
pub const SQ_SIZE: usize = 16; 
pub const SQ_U32: u32 = SQ_SIZE as u32; 
pub const SQ_I32: i32 = SQ_SIZE as i32; 

/// A single unit of the Minesweeper board
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    /// Whether or not this square has a mine
    pub mine: bool,
    /// The number on this mine
    pub num: u8,
    /// Whether the square is open
    pub open: bool,
    /// Whether the square has a flag on it
    pub flag: bool,
}
impl Default for Cell {
    fn default() -> Self { 
        Self {
            mine: false,
            num: 0,
            open: false,
            flag: false,
        }
    }
}

trait MineBoard {
    type IndexShape;

    const NEIGHBORHOOD: [Self::IndexShape];
    const X: i32;
    const Y: i32;
    const SQ_SIZE: usize; 

    fn open(&mut self) -> Cell;
}

/// The Minesweeper board
pub struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}
impl Board {
    pub const X: i32 = 0;
    pub const Y: i32 = 26;
    pub const SQ_SIZE: usize = 16; 
    pub const SQ_U32: u32 = SQ_SIZE as u32; 
    pub const SQ_I32: i32 = SQ_SIZE as i32; 

    const NEIGHBORHOOD: [(i32, i32); 8] = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    /// Converts coordinates in pixels to the corresponding square
    pub fn coord(x: i32, y: i32) -> (usize, usize) {
        let (rel_x, rel_y) = (
            (x - Self::X) as usize, 
            (y - Self::Y) as usize
        );
        (rel_x/Self::SQ_SIZE, rel_y/Self::SQ_SIZE)
    }

    /// Creates a new board.
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![Cell::default(); width*height];

        Self {
            cells,
            width,
            height,
        }
    }

    /// Clears out the board to the default Cell.
    pub fn reset(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self[(x, y)] = Cell::default();
            }
        }
    }

    /// Opens a square. Returns whether or not it loses the game.
    pub fn open(&mut self, p: (usize, usize)) -> bool {
        if !self[p].open && !self[p].flag {
            self[p].open = true; 
            if self[p].num == 0 && !self[p].mine {
                for neighbor in self.get_neighborhood(p) {
                    if !(self[neighbor].open || self[neighbor].mine) {
                        self.open(neighbor);
                    }
                }
            }
            self[p].mine
        } else {
            false
        }
    }

    /// Chords. Returns whether or not it loses the game
    pub fn chord(&mut self, p: (usize, usize)) -> bool {
        if self[p].mine == false && self[p].open {
            let flags = self.get_neighborhood(p)
                .filter(|(x, y)| self[(*x, *y)].flag)
                .count();
            println!("flags: {}, num: {}", flags, self[p].num);
            if self[p].num as usize == flags {
                for neighbor in self.get_neighborhood(p) {
                    if self.open(neighbor) {
                        return true
                    }
                }
            } 
        } 
        false
    }

    pub fn width(&self) -> usize { self.width }

    pub fn height(&self) -> usize { self.height }

    fn get_neighborhood(
        &self, 
        (x, y): (usize, usize)) 
    -> impl Iterator<Item=(usize, usize)> {
        let (width, height) = (self.width() as i32, self.height() as i32);
        let (x, y) = (x as i32, y as i32);
        Self::NEIGHBORHOOD
            .iter()
            .map(move |(dx, dy)| (x+dx, y+dy))
            .filter(move |(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
            .map(|(x, y)| (x as usize, y as usize))
    }

    pub fn generate(&mut self, num_mines: usize, (x, y): (usize, usize)) {
        // generate mines
        let mut mines = vec![false; self.width() * self.height() - 1];
        for i in 0..num_mines {
            mines[i] = true;
        }
        mines.shuffle(&mut thread_rng());

        let skip_idx = y * self.width() + x;
        // assign mines
        for (i, m) in mines.iter().enumerate() {
            if i >= skip_idx {
                self.cells[i+1].mine = *m;
            } else {
                self.cells[i].mine = *m;
            }
        }

        // generate numbers
        for x in 0..self.width {
            for y in 0..self.height {
                self[(x, y)].num = self.get_neighborhood((x, y))
                    .map(|p| self[p])
                    .map(|sq| sq.mine as u8)
                    .sum();
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


#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn neighborhood() {
        let (width, height) = (16, 16);
        let board = Board::new(width, height);

        macro_rules! test_nhood {
            ($pt:expr, $test:expr) => {
                let mut result: Vec<(usize, usize)> = board
                    .get_neighborhood($pt)
                    .collect();
                result.sort();
                let mut test = $test;
                test.sort();
                assert_eq!(result, test);
            }
        }

        // TOP LEFT CORNER
        test_nhood!(
            (0,0),
            vec![
                (0, 1), 
                (1, 0), 
                (1, 1),
            ]
        );

        // BOTTOM RIGHT CORNER
        test_nhood!(
            (width-1, height-1),
            vec![
                (width-2, height-1), 
                (width-1, height-2), 
                (width-2, height-2),
            ]
        );

        // BOTTOM LEFT CORNER
        test_nhood!(
            (0, height-1),
            vec![
                (1, height-1), 
                (0, height-2), 
                (1, height-2),
            ]
        );

        // TOP RIGHT CORNER
        test_nhood!(
            (width-1, 0),
            vec![
                (width-2, 0), 
                (width-1, 1), 
                (width-2, 1),
            ]
        );

        // TOP SIDE
        test_nhood!(
            (8, 0),
            vec![
                (7, 0), 
                (9, 0), 
                (7, 1), 
                (8, 1), 
                (9, 1), 
            ]
        );

        // BOTTOM SIDE
        test_nhood!(
            (8, height-1),
            vec![
                (7, height-1), 
                (9, height-1), 
                (7, height-2), 
                (8, height-2), 
                (9, height-2), 
            ]
        );

        // LEFT SIDE
        test_nhood!(
            (0, 8),
            vec![
                (0, 7), 
                (0, 9), 
                (1, 7), 
                (1, 8), 
                (1, 9), 
            ]
        );
        
        // RIGHT SIDE
        test_nhood!(
            (width-1, 8),
            vec![
                (width-1, 7), 
                (width-1, 9), 
                (width-2, 7), 
                (width-2, 8), 
                (width-2, 9), 
            ]
        );

        // MIDDLE 
        test_nhood!(
            (8, 8),
            vec![
                (7, 7),
                (8, 7),
                (9, 7),
                (7, 8),
                (9, 8),
                (7, 9),
                (8, 9),
                (9, 9),
            ]
        );
    }

    #[test]
    pub fn generate() {
        let (width, height) = (8, 8);
        let mut board = Board::new(width, height);

        println!("== BEFORE ==");
        for x in 0..width {
            for y in 0..height {
                print!("{}", match board[(x, y)].mine {
                    true => 'X',
                    false => '.',
                });
            }
            println!();
        }

        board.generate(10, (0, 0));

        println!("== AFTER ==");
        for x in 0..width {
            for y in 0..height {
                print!("{}", match board[(x, y)].mine {
                    true => 'X',
                    false => '.',
                });
            }
            println!();
        }
    }
}
