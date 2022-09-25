use crossterm::{
    event::{poll, read, Event::*, KeyCode},
    terminal, Result,
};
use std::time::Duration;

fn main() -> Result<()> {
    terminal::enable_raw_mode()?;
    loop {
        let mut c = None;
        if let Ok(true) = poll(Duration::from_millis(100)) {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    c = Some(key_event);
                }
            }
        }
        if let Some(c) = c {
            if c.code == KeyCode::Char('q') {
                break;
            } else {
                println!("{c:?}\r");
            }
        } else {
            println!("no key\r")
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
