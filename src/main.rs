mod termios;
mod raw_mode;

use std::io::{self, Read, Result, Write};
use std::str::from_utf8;

use raw_mode::RawMode;

fn main() -> Result<()> {
    let _raw = RawMode::enable()?;

    loop {
        let mut buffer = [0u8; 1];

        io::stdin().read(&mut buffer)?;

        if buffer == [b'q'] { break };

        if buffer[0].is_ascii_control() {
            print!("{}", buffer[0]);
            io::stdout().flush().unwrap();
        } else {
            print!("{}", from_utf8(&buffer).unwrap());
            io::stdout().flush().unwrap();
        }
    };

    Ok(())
}
