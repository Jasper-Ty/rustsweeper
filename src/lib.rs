pub mod board;
pub mod button;
pub mod draw;
pub mod render;
pub mod action;

pub use action::*;
pub use board::*;
pub use draw::*;
pub use render::*;

#[macro_export]
macro_rules! rect {
    ($x: expr, $y: expr, $w: expr, $h: expr) => {
        Rect::new(($x) as i32, ($y) as i32, ($w) as u32, ($h) as u32)
    };
}

pub trait ClickRect {
    const POS_X: i32;
    const POS_Y: i32;
    const WIDTH: usize;
    const HEIGHT: usize;
    const Z: i32;
}

#[derive(Debug)]
pub enum GameState {
    Init,
    Play,
    Win,
    Lose,
}

pub const BTN_SIZE: usize = 26;
pub const BTN_X: i32 = 0;
pub const BTN_Y: i32 = 0;
