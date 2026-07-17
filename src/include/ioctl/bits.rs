// === Define macros ===
pub const TIOCGWINSZ: i32 = 0x5413; // Argument to get windows size -> /usr/include/asm-generic/ioctl.h

// Declare Winsize struct
#[derive(Default)]
#[repr(C)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16
}