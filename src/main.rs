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

        todo!();
    }
}

fn main() {
    let state = PlayState::IDLE;

}
