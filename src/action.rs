pub enum Click {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

pub enum InputState {
    None,
    Left,
    Right,
    Chord,
}
impl InputState {
    pub fn transition (&mut self, input_action: InputAction, (x, y): (usize, usize)) -> Action {
        match input_action {
            InputAction::LeftUp => {
                match self {
                    InputState::Left => {
                        *self = InputState::None;
                        Action::Open(x, y)
                    },
                    InputState::Chord => {
                        *self = InputState::Right;
                        Action::Chord(x, y)
                    },
                    _ => Action::None,
                }
            }
            InputAction::LeftDown => {
                match self {
                    InputState::None => {
                        *self = InputState::Left;
                        Action::None
                    },
                    InputState::Right => {
                        *self = InputState::Chord;
                        Action::None
                    }
                    _ => Action::None,
                }
            }
            InputAction::RightUp => {
                match self {
                    InputState::Right => {
                        *self = InputState::None;
                        Action::None
                    },
                    InputState::Chord => {
                        *self = InputState::Left;
                        Action::Chord(x, y)
                    },
                    _ => Action::None,
                }
            }
            InputAction::RightDown => {
                match self {
                    InputState::None => {
                        *self = InputState::Right;
                        Action::Flag(x, y)
                    },
                    InputState::Left => {
                        *self = InputState::Chord;
                        Action::None
                    },
                    _ => Action::None,
                }
            }
            InputAction::None => Action::None
        }
    }
}

pub enum InputAction {
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
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

