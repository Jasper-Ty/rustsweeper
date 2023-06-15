use std::error;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rustsweeper::*;

fn main() -> Result<(), String> {
    let board = Board::new_random(30, 16, 50);
    let mut overlay = Overlay::new(30, 16);

    let (mut canvas, mut event_pump) = init_sdl2()?;
    let texture_creator = canvas.texture_creator();
    let spritesheet = Spritesheet::new(&texture_creator)?;

    'running: loop {
        canvas.clear();
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
                            overlay[(x, y)] = Cover::Open;
                            reveal((x, y), &mut overlay, &board);
                        },
                        MouseButton::Right => {
                            overlay[(x, y)] = Cover::Open;
                        }
                        _ => {},
                    }
                },

                _ => {}
            }
        }

        board.render(&mut canvas, &spritesheet)?;
        overlay.render(&mut canvas, &spritesheet)?;

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
        .window("rustsweeper", 30*SQ_U32, 16*SQ_U32)
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
