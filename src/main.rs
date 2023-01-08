use crossterm::{event::Event::*, terminal, Result};
mod editor;
mod input;
mod keyboard;
mod output;
use editor::*;
use input::*;
use output::*;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new()?;
    loop {
        if refresh_screen().is_err() {
            die("Unabled to refresh screen")
        }
        if process_keypress() {
            break;
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
