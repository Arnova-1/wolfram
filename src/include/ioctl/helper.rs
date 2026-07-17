use std::io::{ Result, Error };

use super::bits::{ Winsize, TIOCGWINSZ };
use super::ffi::ioctl;

impl Winsize {
    pub fn from_fd(fd: i32) -> Result<Self> {
        let mut ws = Winsize::default();

        let ret = unsafe {
            ioctl(fd, TIOCGWINSZ, &mut ws)
        };

        if ret == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(ws)
        }
    }
}