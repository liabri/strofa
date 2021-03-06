use std::time::Duration;
use crossterm::event;

pub struct Events;
pub enum Event {
    Input(Key),
}

use futures_util::Stream;

impl Events {
    pub fn new() -> impl Stream<Item = Event> {
        async_stream::stream! {
            loop {
                if event::poll(Duration::from_millis(250)).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        yield Event::Input(Key::from(key))
                    }
                }
            }
        }
    }
}

// normalise stream above, do I need async_stream ?
// todo: use crossterm::KeyEvent instead of this abstraction.

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Key {
    Enter,
    Tab,
    Backspace,
    Esc,
    Left,
    Right,
    Up,
    Down,
    Char(char),
    Ctrl(char),
    Alt(char),
    Unknown
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Key::Alt(' ') => write!(f, "<Alt+Space>"),
            Key::Ctrl(' ') => write!(f, "<Ctrl+Space>"),
            Key::Char(' ') => write!(f, "<Space>"),
            Key::Alt(c) => write!(f, "<Alt+{}>", c),
            Key::Ctrl(c) => write!(f, "<Ctrl+{}>", c),
            Key::Char(c) => write!(f, "{}", c),
            Key::Left | Key::Right | Key::Up | Key::Down => write!(f, "<{:?} Arrow Key>", self),
            Key::Enter | Key::Tab | Key::Backspace | Key::Esc => write!(f, "<{:?}>", self),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {    
            event::KeyEvent { code: event::KeyCode::Esc, .. } => Key::Esc,
            event::KeyEvent { code: event::KeyCode::Backspace, .. } => Key::Backspace,
            event::KeyEvent { code: event::KeyCode::Left, .. } => Key::Left,
            event::KeyEvent { code: event::KeyCode::Right, .. } => Key::Right,
            event::KeyEvent { code: event::KeyCode::Up, .. } => Key::Up,
            event::KeyEvent { code: event::KeyCode::Down, .. } => Key::Down,
            event::KeyEvent { code: event::KeyCode::Enter, .. } => Key::Enter,
            event::KeyEvent { code: event::KeyCode::Tab, .. } => Key::Tab,

            // First check for char + modifier
            event::KeyEvent { code: event::KeyCode::Char(c), modifiers: event::KeyModifiers::ALT } => Key::Alt(c),
            event::KeyEvent { code: event::KeyCode::Char(c), modifiers: event::KeyModifiers::CONTROL } => Key::Ctrl(c),
            event::KeyEvent { code: event::KeyCode::Char(c), .. } => Key::Char(c),
            _ => Key::Unknown,
        }
    }
}