use crate::render::frame::Frame;

pub fn draw_rows(frame: &mut Frame) {
    for row in 0..frame.rows {
        frame.buffer.push('~');

        if row < frame.rows - 1 {
            frame.buffer.push_str("\r\n");
        }
    }
}

pub fn draw_character(frame: &mut Frame, ch: char) {
    frame.buffer.push(ch);
}