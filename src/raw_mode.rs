use crate::termios::bits::{ Termios, ECHO, OPOST, STDIN_FILENO, TCSAFLUSH, ICANON, ISIG, IXON, IEXTEN, ICRNL, BRKINT, INPCK, ISTRIP, CS8, VTIME, VMIN };
use crate::termios::ffi::tcsetattr;

use std::io::{ Result, Error };

#[derive(Debug)]
pub struct RawMode {
    pub original: Termios,
}

impl Drop for RawMode {
    fn drop(&mut self) {
       unsafe { 
           tcsetattr(STDIN_FILENO, TCSAFLUSH, &mut self.original);
       };
    }
}

impl RawMode {
    pub fn enable() -> Result<Self> {
        let original = Termios::from_fd(STDIN_FILENO)?;
        let mut term = original;

        Self::make_raw(&mut term);

        Self::apply(&mut term)?;

        Ok(Self { original })
    }

    fn make_raw(term: &mut Termios) {
        term.c_iflag &= !( IXON | ICRNL | BRKINT | INPCK | ISTRIP );
        term.c_cflag |= CS8;
        term.c_lflag &= !( ECHO | ICANON | ISIG | IEXTEN );
        term.c_oflag &= !OPOST;
        term.c_cc[VMIN] = 0;
        term.c_cc[VTIME] = 1;
    }

    fn apply(term: &mut Termios) -> Result<()> {
       let ret = unsafe {
           tcsetattr(STDIN_FILENO, TCSAFLUSH, term)
       };

       if ret == -1 {
           Err(Error::last_os_error())
       } else {
           Ok(())
       }
    } 
}
