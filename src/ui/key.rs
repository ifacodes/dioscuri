use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Key {
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

impl From<KeyEvent> for Key {
    fn from(key_event: KeyEvent) -> Self {
        match key_event {
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Key::Enter,
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Key::Left,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Key::Right,
            KeyEvent {
                code: KeyCode::Up, ..
            } => Key::Up,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Key::Down,
            KeyEvent {
                code: KeyCode::PageUp,
                ..
            } => Key::PageUp,
            KeyEvent {
                code: KeyCode::PageDown,
                ..
            } => Key::PageDown,
            KeyEvent {
                code: KeyCode::Esc, ..
            } => Key::Esc,
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::ALT,
            } => Key::Alt(c),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
            } => Key::Ctrl(c),
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}
