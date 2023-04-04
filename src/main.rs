//based off of Javidx9's tetris tutorial in C++. (https://youtu.be/8OK8_tHeCIA)

use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, Result, terminal, cursor};
use std::{thread, time};

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

    //leave this as a char (or u8), just convert to a byte it's time to write?
    let mut p_field: [usize; FIELD_WIDTH * FIELD_HEIGHT] = [0; FIELD_WIDTH * FIELD_HEIGHT];

    for x in 0..FIELD_WIDTH {
        for y in 0..FIELD_HEIGHT {
            p_field[y * FIELD_WIDTH + x] = 
            
            if x == 0 || x == FIELD_WIDTH - 1 || y == FIELD_HEIGHT - 1 {
                //9 is the border in Javid's example.
                9
            }

            else {
                0
            }
        }
    };
    
    fn _rotate (px: u8, py: u8, r: u8) -> u8 {
        match r % 4 {
            0 => py * 4 + px,
            1 => 12 + py - (px * 4),
            2 => 15 - (py * 4) - px,
            3 => 3 - py + (px * 4),
            _ => 0,
        }
    }

    fn does_tetromino_fit (id_tetromino: u8, rotation: u8, x_pos: u8, y_pos: u8) -> bool {
        for px in 0..4 {
            for py in 0.4 {
                // gets index into piece
                let pi = rotate(px, py, rotation);

                // gets index into field
                let fi = (y_pos + py) * FIELD_WIDTH + (x_pos + px);
                if (x_pos + px >= 0 && x_pos + px < FIELD_WIDTH) {
                    if (y_pos + py >= 0 && y_pos + py <   FIELD_HEIGHT) {
                        if (tetromino[id_tetromino][pi]
                    }
                }
            }
        } 
        
        true
    };

    let mut buffer = stdout();
    
    buffer.execute(terminal::Clear(terminal::ClearType::All))?;
    buffer.execute(terminal::SetSize(terminal::size()?.0, terminal::size()?.1))?;            
    
    let game_over = false;

    // while game_over == false {

    // // timing
    
    // // input
    
    // // logic

    // // output
    
        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
    
                buffer.execute(cursor::MoveTo((x) as u16, (y) as u16))?;
                buffer.write(
                    // this is an expression that returns a reference to a slice of u8, which is an indexed value of p_field.
                    &[" ABCDEFG=#".as_bytes()[p_field[y * FIELD_WIDTH + x]]]
                ).unwrap();
    
                thread::sleep(time::Duration::from_millis(50));
            }
        }
        // buffer.execute(terminal::Clear(terminal::ClearType::All))?;
    // }
    println!("Terminal size: {:?}", terminal::size()?);

    Ok(())
    
}
