mod include;
mod terminal;
mod input;
mod editor;
mod render;

use std::io::Result;

use terminal::raw::RawMode;
use terminal::input::read_key;
use input::{ parser::{ Parser, ParseResult } , action::map };

use crate::{render::{frame::Frame, renderer::{draw_character, draw_rows}}, terminal::{ansi, output::flush, size::Size}};

fn main() -> Result<()> {
    let _raw = RawMode::enable()?;
    
    let mut editor = editor::state::Editor::default(); 
    let mut buffer = Parser::default();
    
    let size = Size::query()?;
    let mut frame = Frame::new(size);

    draw_rows(&mut frame);
    
    loop {
        let raw_byte = match read_key()? {
            Some(b) => b,
            None => continue,
        };
 
        let parsed_byte = match buffer.feed(raw_byte) {
            ParseResult::Complete(key) => key, 
            ParseResult::Waiting => continue,
            ParseResult::Invalid => break
        };
      
        let action = map(&parsed_byte);

        editor.handle(&action);

        draw_character(&mut frame, editor.character);
      
        flush(&frame)?;
      
        if editor.should_quit {
            ansi::clear_screen()?;
            break;
        }
    }
    
    Ok(())
}
