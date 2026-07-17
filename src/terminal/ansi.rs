use std::io::{ Read, Result, Write, stdin, stdout };

use crate::terminal::size::Size;

pub fn clear_screen() -> Result<()> {
    stdout().write(b"\x1b[2J")?; // \x1b[ = escape sequence, 2 = argument, J = Erase in Display command
    stdout().write(b"\x1b[H")?; // H = Reposition cursor, take two argument (default 1;1)
    
    Ok(())
}

pub fn get_window_size() -> Result<Size> {
    // Move cursor to the bottom-right
    stdout().write(b"\x1b[999C\x1b[999B")?; // C = Cursor forward, B = Cursor down

    let (rows, cols) = get_cursor_position()?;

    Ok(Size {
        rows,
        cols
    })
}

fn get_cursor_position() -> Result<(u16, u16)>{
    let mut buffer = [0u8; 8];

    // Query terminal for cursor position
    stdout().write(b"\x1b[6n")?; // n = Device Status Report, 6 = Argument to query cursor position

    // Receive terminal response
    stdin().read(&mut buffer)?;

    let (rows, cols) = parse_response(&buffer);

    Ok((rows, cols))
}

fn parse_response(buf: &[u8]) -> (u16, u16) {
    let body = &buf[2..buf.len() - 1];
    let pos = body.iter().position(|&b| b == b';').unwrap();

    let rows = body[..pos].iter().fold(
        0u16, |acc, &b| {
            acc * 10 + (b - b'0') as u16
        }
    );
    let cols = body[pos + 1..].iter().fold(
            0u16, |acc, &b| {
                acc * 10 + (b - b'0') as u16
            }
        );

    (rows, cols)
}