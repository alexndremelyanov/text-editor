use crate::keyboard::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn process_keypress() -> bool {
    let c = read_key();
    match c {
        Ok(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => true,
        _ => false,
    }
}
