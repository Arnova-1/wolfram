use crate::{editor::state::Editor, input::action::Action};

impl Editor {
    pub fn handle(&mut self, act: &Action) {
        match act {
            Action::Quit => { self.should_quit = true }
            Action::Insert(c) => { 
               self.character = *c; 
            }
            Action::Nothing => ()
        }
    }
}