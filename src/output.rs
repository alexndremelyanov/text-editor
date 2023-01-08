use crossterm::{cursor, style::Print, terminal, QueueableCommand, Result};
use errno::errno;
use std::io::{stdout, Stdout, Write};

pub fn draw_raws(stdout: &mut Stdout) -> Result<()> {
    for row in 0..24 {
        stdout
            .queue(cursor::MoveTo(0, row))?
            .queue(Print("~".to_string()))?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn clear_screen(stdout: &mut Stdout) -> Result<()> {
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

pub fn refresh_screen() -> Result<()> {
    let mut stdout = stdout();
    clear_screen(&mut stdout)?;
    draw_raws(&mut stdout)?;
    stdout.queue(cursor::MoveTo(0, 0))?.flush()?;
    Ok(())
}

pub fn die<S: Into<String>>(message: S) {
    let mut stdout = stdout();
    let _ = clear_screen(&mut stdout);
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
