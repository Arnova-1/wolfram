use crate::terminal::size::Size;

pub struct Frame {
    pub buffer: String,
    pub rows: u16,
    pub cols: u16
}

impl Frame {
    pub fn new(sz: Size) -> Self {
        let buffer = String::new();
        let rows = sz.rows;
        let cols = sz.cols;

        Self {
            buffer,
            rows,
            cols
        }
    }
}