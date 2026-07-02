mod termios_bits;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = [0u8; 1];

    while io::stdin().read(&mut buffer)? == 1 && buffer != [b'q'] {};

    Ok(())
}
