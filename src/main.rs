use std::ops::{ Index, IndexMut };
use std::error;
use std::time::Duration;

use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{ thread_rng };

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::{ Color };
use sdl2::surface::Surface;

use rustsweeper::Spritesheet;
use rustsweeper::Sprite;

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
                self[(x, y)] = Cell::Num(
                    neighborhood.iter()
                        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                        .filter(|(x, y)| 
                            *x > 0 && *x < self.width as i32
                            && *y > 0 && *y < self.height as i32)
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

const SQ_SIZE: usize = 48; 
const SQ_I32: i32 = SQ_SIZE as i32;
const SQ_U32: u32 = SQ_SIZE as u32;

fn main() -> Result<(), Box<dyn error::Error>> {
    let state = PlayState::IDLE;

    let board = Board::new_random(30, 16, 99);

    let (mut canvas, mut event_pump) = init_sdl2()?;
    let texture_creator = canvas.texture_creator();

    let spritesheet = Spritesheet::new(&texture_creator)?;


    canvas.set_draw_color(Color::RGB(255, 0, 0));
    for y in 0..16 {
        for x in 0..30 {
            canvas.draw_rect(Rect::new(x*SQ_I32, y*SQ_I32, SQ_U32, SQ_U32))?;
        }
    }

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    for y in 0..16 {
        for x in 0..30 {
            let rect = Rect::new(x as i32 * SQ_I32, y as i32 * SQ_I32, SQ_U32, SQ_U32);
            match board[(x, y)] {
                Cell::Mine => {
                    spritesheet.draw(&mut canvas, Sprite::Mine, rect); 
                },
                Cell::Num(n) => {
                    spritesheet.draw(&mut canvas, Sprite::Num(n), rect);
                },
            }
        }
    }
    canvas.present();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{ .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


    Ok(())
}

use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
fn init_sdl2() -> Result<(Canvas<Window>, EventPump), Box<dyn error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rustsweeper", 30*SQ_U32, 16*SQ_U32)
        .resizable()
        .position_centered()
        .opengl()
        .build()?;

    let canvas = window.into_canvas()
        .present_vsync()
        .build()?;
    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}
