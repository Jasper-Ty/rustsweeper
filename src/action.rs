use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use crate::*;

pub enum Click {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

pub enum InputState {
    None,
    Left(usize, usize),
    Right,
    Chord(usize, usize),
}
impl InputState {
    pub fn transition (&mut self, input_action: InputAction) -> Action {
        match input_action {
            InputAction::LeftUp(x, y) => {
                match self {
                    InputState::Left(..) => {
                        *self = InputState::None;
                        Action::Open(x, y)
                    },
                    InputState::Chord(..) => {
                        *self = InputState::None;
                        Action::Chord(x, y)
                    },
                    _ => Action::None,
                }
            }
            InputAction::LeftDown(x, y) => {
                match self {
                    InputState::None => {
                        *self = InputState::Left(x, y);
                        Action::None
                    },
                    InputState::Right => {
                        *self = InputState::Chord(x, y);
                        Action::None
                    }
                    _ => Action::None,
                }
            }
            InputAction::RightUp(x, y) => {
                match self {
                    InputState::Right => {
                        *self = InputState::None;
                        Action::None
                    },
                    InputState::Chord(..) => {
                        *self = InputState::None;
                        Action::Chord(x, y)
                    },
                    _ => Action::None,
                }
            }
            InputAction::RightDown(x, y) => {
                match self {
                    InputState::None => {
                        *self = InputState::Right;
                        Action::Flag(x, y)
                    },
                    InputState::Left(..) => {
                        *self = InputState::Chord(x, y);
                        Action::Flag(x, y)
                    },
                    _ => Action::None,
                }
            }
            InputAction::None => Action::None
        }
    }
}
pub enum InputAction {
    LeftUp(usize, usize),
    LeftDown(usize, usize),
    RightUp(usize, usize),
    RightDown(usize, usize),
    None,
}

pub enum Action {
    Chord (usize, usize),
    Flag (usize, usize),
    Open (usize, usize),
    Btn,
    Quit,
    None,
}

