use super::key::Key;

#[derive(Default)]
pub struct Parser {
    buffer: [u8; 4],
    received: usize,
    expected: usize
}

pub enum ParseResult {
    Waiting,
    Complete(Key),
    Invalid
}

impl Parser {
    pub fn feed(&mut self, byte: u8) -> ParseResult {
        if let Some(key) = Self::decode_ctrl(&byte) {
            return ParseResult::Complete(key);
        }
        
        if self.received == 0 {
            self.initialize_sequence(&byte);
        }

        self.buffer[self.received] = byte;

        self.expected -= 1;
        self.received += 1;

        if self.expected == 0 {
            let result = self.decode_utf8();
            self.received = 0;
            return result;
        }
        
        ParseResult::Waiting
    }

    fn initialize_sequence(&mut self, byte: &u8) {
        match byte {
            0x00..=0x7F => self.expected = 1,
            0xC0..=0xDF => self.expected = 2,
            0xE0..=0xEF => self.expected = 3,
            0xF0..=0xF7 => self.expected = 4,
            _ => ()
        }
    }

    fn decode_ctrl(byte: &u8) -> Option<Key> {
        match byte {
            1..=26 => Some(Key::Ctrl(byte + b'a' - 1)),
            _ => None
        }
    }

    fn decode_utf8(&mut self) -> ParseResult {
        match std::str::from_utf8(&self.buffer[..self.received]) {
            Ok(s) => {
                let character: char = s.chars().next().unwrap();
                return ParseResult::Complete(Key::Character(character));
            },
            Err(_) => {
                return ParseResult::Invalid;
            }
        }
    }
}