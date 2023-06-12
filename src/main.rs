use sdl2;

enum PlayState {
    IDLE,
    PLAY,
    WIN,
    GAMEOVER,
}

enum Cell {
    MINE,
    NUM(u8),
}

struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new_random(width: usize, height: usize, num_mines: usize) -> Self {
        let cells: Vec<Cell> = Vec::with_capacity(width * height);
        todo!();
    }

    fn gen_mines(&mut self, num_mines: usize) {

        todo!();
    }
    fn gen_numbers(&mut self, num_mines: usize) {

        todo!();
    }
}

fn main() {
    let state = PlayState::IDLE;

}
