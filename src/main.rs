use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::event::EventType;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rustsweeper::*;

pub enum Click {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}
pub enum Action {
    BoardClick {
        click: Click,
        board_x: usize,
        board_y: usize,
    },
    BtnClick,
    Quit,
    None,
}

fn main() -> Result<(), String> {

    let mut state = GameState::Init;

    let width = 30;
    let height = 16;
    let num_mines = 50;

    let mut board = Board::new(width, height);

    let (mut canvas, mut event_pump) = init_sdl2(width, height)?;
    let texture_creator = canvas.texture_creator();
    let spritesheet = Spritesheet::new(&texture_creator)?;

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            let action = match event {
                /* Exit */
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => Action::Quit,

                /* Click */
                Event::MouseButtonUp {
                    mouse_btn, x,y,
                    ..
                } 
                | Event::MouseButtonDown {
                    mouse_btn, y, x,
                    ..
                }=> {
                    let click = match (event, mouse_btn) {
                        (Event::MouseButtonUp {..}, MouseButton::Left) => Click::LeftUp,
                        (Event::MouseButtonDown {..}, MouseButton::Left) => Click::LeftDown,
                        (Event::MouseButtonUp {..}, MouseButton::Right) => Click::LeftUp,
                        (Event::MouseButtonDown {..}, MouseButton::Right) => Click::LeftDown,
                        _ => Click::LeftDown,
                    };
                    let board_rect = rect!(
                        BOARD_X, 
                        BOARD_Y, 
                        board.width()*SQ_SIZE,
                        board.height()*SQ_SIZE
                    );
                    let btn_rect = rect!(
                        BTN_X,
                        BTN_Y,
                        BTN_SIZE,
                        BTN_SIZE
                    );
                    if board_rect.contains_point((x, y)) {
                        let (board_x, board_y) = { 
                            let (rel_x, rel_y) = (
                                (x - BOARD_X) as usize, 
                                (y - BOARD_Y) as usize
                            );
                            (rel_x/SQ_SIZE, rel_y/SQ_SIZE)
                        };
                        Action::BoardClick { click, board_x, board_y }
                    } else if btn_rect.contains_point((x, y)) {
                        Action::BtnClick
                    } else {
                        Action::None
                    }
                },

                _ => Action::None 
            };

            match &action { 
                Action::None => {},
                _ => println!("state: {:?}", state),
            }

            match (&state, action) {
                (_, Action::Quit) => break 'running,
                (GameState::Init, Action::BoardClick { click,  board_x, board_y }) => {
                    match click {
                        Click::LeftUp => {
                            state = GameState::Play;
                            board.generate(num_mines, (board_x, board_y));
                            board.open((board_x, board_y));
                        }
                        _ => {}
                    }
                },
                (GameState::Play, Action::BoardClick { click, board_x, board_y }) => {
                    match click {
                        Click::LeftUp => {
                            board.tentative = None;
                            let cell = board.open((board_x, board_y));
                            if cell.mine {
                                state = GameState::Lose;
                            }
                        },
                        Click::LeftDown => {
                            board.tentative = Some((board_x, board_y));
                        }
                        _ => {}
                    }
                },
                (GameState::Play, Action::BtnClick) 
                | (GameState::Lose, Action::BtnClick) => {
                    state = GameState::Init;
                    board.reset();
                },
                (_, _) => {}
            }

        }

        
        spritesheet.draw(&mut canvas, Sprite::BtnSmile, rect!(BTN_X, BTN_Y, BTN_SIZE, BTN_SIZE))?;

        match board.render(&mut canvas, &spritesheet) {
            Err(_) => break 'running,
            _ => {}
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }


    Ok(())
}

use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;


fn init_sdl2(width: usize, height: usize) -> Result<(Canvas<Window>, EventPump), String> {
    let (width, height) = (width as u32, height as u32);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rustsweeper", 32+width*SQ_U32, 32+height*SQ_U32)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    let disabled_events = [
        EventType::MouseMotion,
        EventType::Window,
        EventType::TextEditing,
    ];
    for event_type in disabled_events {
        event_pump.disable_event(event_type);
    }

    Ok((canvas, event_pump))
}
