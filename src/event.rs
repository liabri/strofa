use std::{ sync::mpsc, thread, time::Duration };
use crossterm::event;

#[derive(Debug, Clone, Copy)]
pub struct EventConfig {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            exit_key: Key::Ctrl('c'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

pub enum Event<I> {
    Input(I),
    Tick
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    _tx: mpsc::Sender<Event<Key>>
}


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

impl Events {
    pub fn new() -> Events {
        Events::with_config(EventConfig::default())
    }

    pub fn with_config(config: EventConfig) -> Events {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                if event::poll(config.tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        //doesnt work
                        if key==config.exit_key {
                            std::process::exit(0x0100);
                        }

                        event_tx.send(Event::Input(key)).unwrap();
                    }
                }

                event_tx.send(Event::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> { self.rx.recv() }
}