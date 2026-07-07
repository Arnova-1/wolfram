use crate::terminal::types::Key;

pub fn parse(byte: u8) -> Key {
    if byte <= 26 {
        Key::Ctrl(byte + b'a' - 1)
    } else {
        Key::Character(byte)
    }
}
 
