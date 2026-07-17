// === Declare types ===
type TcFlag = u32;
type Speed = u32;
type Cc = u8;

// === Define macros ===
// NCCS -> /usr/include/bits/termios-struct.h
const NCCS: usize = 32;

// tcsetattr flag -> /usr/include/bits/termios-tcflow.h
pub const TCSAFLUSH: i32 = 2;

// File descriptor -> /usr/include/unistd.h
pub const STDIN_FILENO: i32 = 0;
pub const STDOUT_FILENO: i32 = 1;

// Local mode flags -> /usr/include/bits/termios-c_lflag.h
pub const ECHO: TcFlag = 0o0000010; // Enable Echo (Immediately print characters typed on the keyboard)
pub const ICANON: TcFlag = 0o0000002; // Canonical input (Input processed line by line)
pub const ISIG: TcFlag = 0o0000001; // Enable signal (CTRL-Z & CTRL-C)
pub const IEXTEN: TcFlag = 0o0100000; // Enable implementation-defined input processing (CTRL-V - Print literal bytes)

// Input flags -> /usr/include/bits/termios-c_iflag.h
pub const IXON: TcFlag = 0o0002000; // Enable start/stop output control (CTRL-S & CTRL-Q)
pub const ICRNL: TcFlag = 0o0000400; // Map carriage return (/r) to new line (/n) (CTRL-M returned byte 13 (/r) which get mapped to byte 10 (/n) which is CTRL-J and Enter key)
pub const BRKINT: TcFlag = 0o0000002;
pub const INPCK: TcFlag = 0o0000020;
pub const ISTRIP: TcFlag = 0o0000040;

// Output flags -> /usr/include/bits/termios-c_oflag.h
pub const OPOST: TcFlag = 0o0000001; // Post process output (disable automatic carriage return & newline combinations)

// C flags -> /usr/include/bits/termios-c_cflag.h
pub const CS8: TcFlag = 0o0000060;

// Cc flags -> /usr/include/bits/termios-c_cc.h
pub const VMIN: usize = 6;
pub const VTIME: usize = 5;

// === Declare Termios struct ===
#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct Termios {
    pub c_iflag: TcFlag, // Input flag
    pub c_oflag: TcFlag, // Output flag
    pub c_cflag: TcFlag, // Control flag
    pub c_lflag: TcFlag, // Local flag
    pub c_line: Cc, // Line discipline
    pub c_cc: [Cc; NCCS], // Control spaces
    pub c_ispeed: Speed,
    pub c_ospeed: Speed,
}