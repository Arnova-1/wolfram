use super::bits::Termios;

unsafe extern "C" {
    pub fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
    pub fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *mut Termios) -> i32;
}