#[derive(Debug)]
pub enum Key {
    Character(u8),
    Ctrl(u8),
}

pub enum Event {
    Quit,
    Continue,
}
