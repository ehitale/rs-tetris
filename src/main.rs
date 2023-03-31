use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, QueueableCommand, Result, terminal, style::{self, Stylize}, cursor};

mod experiment;

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 18;

fn main() -> Result<()> {
    
    let mut tetromino: Vec<String> = vec![
        String::new(), 
        String::new(), 
        String::new(), 
        String::new(), 
        String::new(), 
        String::new(), 
        String::new(), 
        String::new()
    ];
    
    tetromino[0].push_str("..X...X...X...X.");
    tetromino[1].push_str("..X..XX...X.....");
    tetromino[2].push_str(".....XX..XX.....");
    tetromino[3].push_str("..X..XX..X......");
    tetromino[4].push_str(".X...XX...X.....");
    tetromino[5].push_str(".X...X...XX.....");
    tetromino[6].push_str("..X...X..XX.....");

    //couldn't I just use a 'char' array? what do I need the Option for?
    let mut p_field: [u8; FIELD_WIDTH * FIELD_HEIGHT] = [b' '; FIELD_WIDTH * FIELD_HEIGHT];

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            p_field[y * FIELD_WIDTH + x] = 
            
            if x == 0 || x == FIELD_WIDTH - 1 || y == FIELD_HEIGHT {
                //'9' is the border in Javid's example.
                b'9'
            }

            else {
                //this should be whitespace.
                b' '
            }
        }
    };
    
    fn _rotate (px: u32, py: u32, r: u32) -> Option<u32> {
        match r % 4 {
            0 => Some(py * 4 + px),
            1 => Some(12 + py - (px * 4)),
            2 => Some(15 - (py * 4) - px),
            3 => Some(3 - py + (px * 4)),
            _ => None,    
        }
    }

    let mut buffer = stdout();

    let game_over = false;

    
    // while !game_over == true {

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            buffer
    // first execute should contain buffer dimensions.
                .execute(cursor::MoveTo(x as u16,y as u16))?;
            
            buffer.write(
                " ABCDEFG=#"[p_field[y * FIELD_WIDTH + x]]
            );
        }
    }
        buffer.execute(terminal::Clear(terminal::ClearType::All))?;
    
        buffer
            .execute(terminal::SetSize(terminal::size()?.0, terminal::size()?.1))?
            .execute(cursor::MoveTo(0, 0))?;
    // }
    // experiment::clear_everything();
    println!("\nTerminal size: {:?}", terminal::size()?);

    Ok(())
    
}
