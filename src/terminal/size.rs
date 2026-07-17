use std::io::Result;

use crate::include::ioctl::bits::Winsize;
use crate::terminal::ansi;
use crate::include::termios::bits::STDOUT_FILENO;

#[derive(Debug)]
pub struct Size {
    pub rows: u16,
    pub cols: u16,
}

impl Size {
    pub fn query() -> Result<Self> {
        match Winsize::from_fd(STDOUT_FILENO) {
            Ok(ws) if ws.ws_row != 0 && ws.ws_col != 0 => {
                Ok(Self::from(ws))
            }
            _ => ansi::get_window_size()
        }
    }
}

impl From<Winsize> for Size {
    fn from(ws: Winsize) -> Self {
        Self {
            rows: ws.ws_row,
            cols: ws.ws_col
        }
    }
}