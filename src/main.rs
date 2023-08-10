use std::time::Duration;
use sdl2::rect::Rect;
use sdl2::event::EventType;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rustsweeper::*;

fn main() -> Result<(), String> {

    let mut game_state = GameState::Init;
    let mut input_state = InputState::None;

    let width = 30;
    let height = 16;
    let num_mines = 50;

    let mut board = Board::new(width, height);
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

    let (mut canvas, mut event_pump) = init_sdl2(width, height)?;
    let texture_creator = canvas.texture_creator();
    let spritesheet = Spritesheet::new(&texture_creator)?;

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            let action = match &event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape) | Some(Keycode::Q),
                    ..
                } => Action::Quit,

                edge @ Event::MouseButtonUp { mouse_btn, x, y, .. } 
                | edge @ Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if board_rect.contains_point((*x, *y)) {
                        let input_action = match (edge, mouse_btn) {
                            (Event::MouseButtonUp {..}, MouseButton::Left) 
                                => InputAction::LeftUp,
                            (Event::MouseButtonDown {..}, MouseButton::Left) 
                                => InputAction::LeftDown,
                            (Event::MouseButtonUp {..}, MouseButton::Right) 
                                => InputAction::RightUp,
                            (Event::MouseButtonDown {..}, MouseButton::Right) 
                                => InputAction::RightDown,
                            _ => InputAction::None,
                        };

                        let (x, y) = Board::coord(*x, *y);                   
                        input_state.transition(input_action, (x, y))
                    } else if btn_rect.contains_point((*x, *y)) {
                        Action::Btn
                    } else {
                        Action::None
                    }
                },

                _ => Action::None 
            };

            match (&game_state, action) {
                (_, Action::Quit) => break 'running,
                (GameState::Init, Action::Open(x, y)) => {
                    game_state = GameState::Play;
                    board.generate(num_mines, (x, y));
                    board.open((x, y));
                },
                (GameState::Play, Action::Open(x, y)) => {
                    if board.open((x, y)) {
                        game_state = GameState::Lose;
                    }
                },
                (GameState::Play, Action::Chord(x, y)) => {
                    if board.chord((x, y)) {
                        game_state = GameState::Lose;
                    }
                },
                (GameState::Play, Action::Flag(x, y)) => {
                    if board[(x, y)].open == false {
                        board[(x, y)].flag = !board[(x, y)].flag;
                    }
                },
                (GameState::Play, Action::Btn) 
                | (GameState::Lose, Action::Btn) => {
                    game_state = GameState::Init;
                    board.reset();
                },
                (_, _) => {}
            }
        }

        let mouse_state = event_pump.mouse_state();
        let (x, y) = Board::coord(mouse_state.x(), mouse_state.y());
        spritesheet.draw(&mut canvas, Sprite::BtnSmile, rect!(BTN_X, BTN_Y, BTN_SIZE, BTN_SIZE))?;

        board.render(&mut canvas, &spritesheet, &game_state, &input_state)?;
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
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
