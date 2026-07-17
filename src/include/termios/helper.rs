use super::bits::Termios;
use super::ffi::tcgetattr;

use std::io::{ Result, Error };

impl Termios {
    pub fn from_fd(fd: i32) -> Result<Self> {
        let mut termios = Self::default();

        let ret = unsafe {
            tcgetattr(fd, &mut termios)
        };

        if ret == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(termios)
        }
    }
}