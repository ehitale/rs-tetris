use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, terminal, terminal::{Clear, ClearType, size}};

pub fn clear_everything () {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All));
    println!("{:?}", size());
}