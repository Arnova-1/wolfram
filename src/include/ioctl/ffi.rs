unsafe extern "C" {
    pub fn ioctl(fd: i32, request: i32, ...) -> i32;
}