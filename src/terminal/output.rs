use std::io::{Result, Write, stdout};

use crate::render::frame::Frame;

pub fn flush(frame: &Frame) -> Result<()> {
    let mut out = stdout().lock();
    
    out.write_all(b"\x1b[2J")?;
    out.write_all(b"\x1b[H")?;
    
    out.write_all(frame.buffer.as_bytes())?;
    out.flush()?;
    
    Ok(())
}