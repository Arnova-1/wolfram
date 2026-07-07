mod termios;
mod terminal;

use std::io::Result;

use crate::terminal::{ raw_mode, keypress };
use crate::terminal::types::Event;

fn main() -> Result<()> {
    let _raw = raw_mode::RawMode::enable()?;

    loop {
        let ret = keypress::process()?;

        match ret {
            Event::Quit => break,
            Event::Continue => continue
        }
    }

    Ok(())
}
