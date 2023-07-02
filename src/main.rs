use std::error;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rustsweeper::*;

pub enum GameState {
    Init,
    Play,
    Win,
    Lose,
}

fn main() -> Result<(), String> {

    let mut state = GameState::Init;

    let width = 30;
    let height = 16;
    let num_mines = 10;

    let mut board = Board::new(width, height);

    let (mut canvas, mut event_pump) = init_sdl2()?;
    let texture_creator = canvas.texture_creator();
    let spritesheet = Spritesheet::new(&texture_creator)?;

    let to_open = width * height - num_mines;

    'running: loop {
        canvas.clear();

        let board_click = false;
        let btn_click = false;

        for event in event_pump.poll_iter() {
            match event {

                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::MouseButtonUp {
                    mouse_btn,
                    x,
                    y,
                    ..
                } => {
                    let (x, y) = (x / SQ_I32, y / SQ_I32);
                    let (x, y) = (x as usize, y as usize);
                    match mouse_btn {
                        MouseButton::Left => {
                            if let GameState::Init = state {
                                board.generate(num_mines, (x, y));
                                state = GameState::Play;
                            }
                            let sq = &mut board[(x, y)];
                            if sq.open == false {
                                if sq.mine {
                                    println!("GAME OVER");
                                }
                                sq.open = true;
                            }
                        },
                        MouseButton::Right => {
                            /*
                            match overlay[(x, y)] {
                                Cover::Closed => overlay[(x,y)] = Cover::Flag,
                                _ => {}
                            }
                            */
                        }
                        _ => {},
                    }
                },

                _ => {}
            }
            
        }
        board.render(&mut canvas, &spritesheet);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


    Ok(())
}

use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
fn init_sdl2() -> Result<(Canvas<Window>, EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rustsweeper", 32+30*SQ_U32, 32+16*SQ_U32)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;

    Ok((canvas, event_pump))
}
