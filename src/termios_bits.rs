// Define type
type TcFlag = u32; // tcflag_t = unsigned int (u32)
type Cc = u8; // cc_t = unsigned char (u8)

// Define Macros
const NCCS: usize = 32;
pub const ECHO: TcFlag = 0o0000010;

// Declare termios struct
#[repr(C)]
#[derive(Default)]
pub struct Termios {
    c_iflag: TcFlag, // Input mode flags
    c_oflag: TcFlag, // Output mode flags
    c_cflag: TcFlag, // Control mode flags
    c_lflag: TcFlag, // Local mode flags
    c_line: Cc, // Line discipline (char)
    c_cc: [Cc; NCCS] // Control spaces
}
