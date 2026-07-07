use std::io::Result;

use crate::terminal::{ parser, input };
use crate::terminal::types::{ Key, Event };

pub fn process() -> Result<Event> {
    let byte = input::read_key()?;
    let key = parser::parse(byte);

    match key {
        Key::Ctrl(b'q') => Ok(Event::Quit),
        _ => { println!("Parsed byte = {:?}\r\n", key); Ok(Event::Continue) }
    }
}
