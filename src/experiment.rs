use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, QueueableCommand, Result};
use crossterm::{terminal, style::{self, Stylize}, cursor};

//function currently returns '()'
pub fn clear_everything () -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..30 {
        for x in 0..50 {
            if (y == 0 || y == 30 - 1) || (x == 0 || x == 50 - 1) {
                stdout
                .execute(cursor::MoveTo(x, y))?
                .execute(style::PrintStyledContent(" ".on(style::Color::Cyan)))?;
            }
        }
    }
    // stdout.flush()?;
    Ok(())
}