use std::io::{Write, stdout, Stdout};
use crossterm::{ExecutableCommand, Result};
use crossterm::{terminal::*, style::*};

//function currently returns '()'
pub fn clear_everything () -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?.execute(SetBackgroundColor(Color::Blue));

    Ok(())
}