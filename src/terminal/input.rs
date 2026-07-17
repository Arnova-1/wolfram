use std::io::{Read, Result, stdin};

pub fn read_key() -> Result<Option<u8>> {
    let mut byte = [0u8; 1];

    match stdin().read(&mut byte)? {
        0 => Ok(None),
        _ => Ok(Some(byte[0])),
    }
}