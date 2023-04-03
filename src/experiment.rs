use std::io::{stdout};
use crossterm::{ExecutableCommand, Result};
use crossterm::{terminal, style::{self, Stylize}, cursor};

pub fn clear_everything () -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    
    let size = match terminal::size() {
        Ok((x, y)) => (x, y),
        Err(e) => panic!("couldn't find terminal dimensions"),
    };

    for y in 0..size.1 {
        for x in 0..size.0 {
            if (y == 0 || y == size.1 - 1) || (x == 0 || x == size.0 - 1) {
                stdout
                .execute(cursor::MoveTo(x, y))?
                .execute(style::PrintStyledContent("a".cyan()))?;
            }
        }
    }
    // stdout.flush()?;
    Ok(())
}