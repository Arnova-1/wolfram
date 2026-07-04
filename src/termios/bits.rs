// Define type
type TcFlag = u32; // tcflag_t = unsigned int (u32)
type Cc = u8; // cc_t = unsigned char (u8)
type Speed = u32; // speed_t = unsigned int (u32)

// === Define Macros ===
// NCCS -> /usr/include/bits/termios-struct.h
const NCCS: usize = 32;

// tcsetattr flag -> /usr/include/bits/termios-tcflow.h
pub const TCSAFLUSH: i32 = 2;

// File descriptor -> /usr/include/unistd.h
pub const STDIN_FILENO: i32 = 0;

// Local mode flags -> /usr/include/bits/termios-c_lflag.h
pub const ECHO: TcFlag = 0o0000010; // Enable echo (immediately print characters typed on the keyboard)
pub const ICANON: TcFlag = 0o0000002; // Canonical input (Input processed line by line)
pub const ISIG: TcFlag = 0o0000001; // Enable signal (CTRL-Z & CTRL-C)
pub const IEXTEN: TcFlag = 0o0100000; // Enable implementation-defined input processing (CTRL-V Print literal bytes)

// Input flags -> /usr/include/bits/termios-c_iflag.h
pub const IXON: TcFlag = 0o0002000; // Enable start/stop output control (CTRL-S & CTRL-Q)
pub const ICRNL: TcFlag = 0o0000400; // Map carriage return (/r) to new line (/n) (CTRL-M returned byte 13 (/r) which get mapped to byte 10 (/n) (CTRL-J and enter key)
pub const BRKINT: TcFlag = 0o0000002;
pub const INPCK: TcFlag = 0o0000020;
pub const ISTRIP: TcFlag = 0o0000040;

// Output flags -> /usr/include/bits/termios-c_oflag.h 
pub const OPOST: TcFlag = 0o0000001; // Post process output (disable automatic carriage return & new line combinations)

// C flags -> /usr/include/bits/termios-c_cflag.h
pub const CS8: TcFlag = 0o0000060;

// Cc flags -> /usr/include/bits/termios-c_cc.h
pub const VMIN: usize = 6;
pub const VTIME: usize = 5;

// Declare termios struct
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct Termios {
    pub c_iflag: TcFlag, // Input mode flags
    pub c_oflag: TcFlag, // Output mode flags
    pub c_cflag: TcFlag, // Control mode flags
    pub c_lflag: TcFlag, // Local mode flags
    pub c_line: Cc, // Line discipline (char)
    pub c_cc: [Cc; NCCS], // Control spaces
    pub c_ispeed: Speed,
    pub c_ospeed: Speed,
}
