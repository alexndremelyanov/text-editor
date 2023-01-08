use crossterm;
use crossterm::Result;

pub struct Editor {
    width: u16,
    height: u16,
}

impl Editor {
    pub fn new() -> Result<Self> {
        let (columns, rows) = crossterm::terminal::size()?;
        Ok(Self {
            width: columns,
            height: rows,
        })
    }
}
