use super::key::Key;

pub enum Action {
    Quit,
    Insert(char),
    Nothing
}

pub fn map(key: &Key) -> Action {
    match key {
        Key::Ctrl(b'q') => Action::Quit,
        Key::Ctrl(_) => Action::Nothing,
        Key::Character(c) => Action::Insert(*c)
    }
}