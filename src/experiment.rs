use crossterm::event::{read, Event};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal,
};
use crossterm::{ExecutableCommand, Result};
use std::io::{stdout, Write};

// use std::{thread, time}

const FIELD_WIDTH: u8 = 12;
const FIELD_HEIGHT: u8 = 12;

pub fn _alternate_screen() -> Result<()> {
    let mut stdout = stdout();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(terminal::EnterAlternateScreen)?;

    // thread::sleep(time::Duration::from_millis(3000));

    stdout.write(b"Can you see me?")?;

    // thread::sleep(time::Duration::from_millis(3000));

    stdout.execute(terminal::LeaveAlternateScreen)?;
    // stdout.flush()?;
    Ok(())
}

pub fn _keyboard_events() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    terminal::enable_raw_mode().unwrap();

    // let size = match terminal::size() {
    //    Ok((x, y)) => (x, y),
    //    Err(_) => panic!("couldn't find terminal dimensions"),
    // };

    // loop {
    //     match read()? {
    //         Event::Key(event) => println!("{:?}", event.code),
    //         _ => (),
    //     }
    // }

    Ok(())
}

pub fn _no_library() -> Result<()> {
    let mut p_field: &mut [u8] = &mut [0; 144];

    for x in 0..12 {
        for y in 0..12 {
            p_field[(y * 12 + x) as usize] = if x == 0 || x == 11 - 1 || y == 11 - 1 {
                9
            } else {
                0
            }
        }
    }

    // let mut screen_buffer: &mut [u8] = &mut [65; 144];
    let mut stdout = stdout();

    for x in 0..12 {
        for y in 0..12 {
            stdout.write(&[" ABCDEFG=#".as_bytes()[p_field[(y * 12 + x) as usize] as usize]])?;
        }
    }

    // stdout.write(screen_buffer)?;

    // questions to ask, what does flush do and why do I need to do it?
    // how do I implement cursor movement/can anyone explain this code? show crossterm cursor movement code.
    Ok(())
}

pub fn _no_library_extended() -> Result<()> {
    let p_field: &mut [u8] = &mut [0; 144];

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            p_field[(y * FIELD_WIDTH + x) as usize] =
                if y == 0 || x == FIELD_WIDTH - 1 || y == FIELD_HEIGHT - 1 {
                    9
                } else {
                    0
                }
        }
    }

    // let mut screen_buffer: &mut [u8] = &mut [65; 144]; // look into Cursors; that seek function is looking might useful. (doesn't seem necessary anymore)
    let mut stdout = stdout();

    stdout.write(b"\x1B[2J").unwrap();

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            stdout.write(
                format!("\x1B[{};{}H", x + 1, y + 1).as_bytes()    
            ).unwrap();
            stdout.write(&[
                " ABCDEFG=#".as_bytes()[p_field[(y * FIELD_WIDTH + x) as usize] as usize]
            ])?;
        }
    }
    stdout.write(b"\x1B[0;0H").unwrap();
    stdout.write(b"That's all folks!\n").unwrap();

    // stdout.write(screen_buffer)?;
    stdout.flush()?;
    Ok(())
}
