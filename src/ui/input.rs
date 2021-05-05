use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Input {
    Enter,
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    Esc,
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown,
}

impl From<KeyEvent> for Input {
    fn from(key_event: KeyEvent) -> Self {
        match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Input::Enter,
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Input::Left,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Input::Right,
            KeyEvent {
                code: KeyCode::Up, ..
            } => Input::Up,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Input::Down,
            KeyEvent {
                code: KeyCode::PageUp,
                ..
            } => Input::PageUp,
            KeyEvent {
                code: KeyCode::PageDown,
                ..
            } => Input::PageDown,
            KeyEvent {
                code: KeyCode::Esc, ..
            } => Input::Esc,
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::ALT,
            } => Input::Alt(c),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
            } => Input::Ctrl(c),
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Input::Char(c),
            _ => Input::Unknown,
        }
    }
}
