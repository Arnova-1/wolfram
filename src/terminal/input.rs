use std::io::{self, Read, Result};

pub fn read_key() -> Result<u8> {
    let mut c = [0u8; 1];

    io::stdin().read(&mut c)?;

    Ok(c[0])
}
