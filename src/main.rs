//based off of Javidx9's tetris tutorial in C++. (https://youtu.be/8OK8_tHeCIA)

use std::io::{Write, stdout};
use crossterm::{ExecutableCommand, Result, terminal, cursor};
use std::{thread, time};

mod experiment;

const FIELD_WIDTH: usize = 12;
const FIELD_HEIGHT: usize = 18;

fn main() -> Result<()> {
    

    
    // let mut tetromino: [String; 7] = [
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from() 
    // ];
    

    //leave this as a char (or u8), just convert to a byte it's time to write?
    let mut tetromino: [&str; 7] = [
        "..X...X...X...X.", 
        "..X..XX...X.....", 
        ".....XX..XX.....", 
        "..X..XX..X......", 
        ".X...XX...X.....", 
        ".X...X...XX.....", 
        "..X...X..XX....." 
    ];

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

    let mut buffer = stdout();
    
    buffer.execute(terminal::Clear(terminal::ClearType::All))?;
    buffer.execute(terminal::SetSize(terminal::size()?.0, terminal::size()?.1))?;            

    let current_piece = 2;
    let current_rotation = 0;
    let current_x = FIELD_WIDTH as u8 / 2;
    let current_y = 0;
    
    let game_over = false;

    while game_over == false {

    // timing
    
    // input
    
    // logic

    // output

        // draw field
        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
    
                buffer.execute(cursor::MoveTo((x) as u16, (y) as u16))?;
                buffer.write(
                    // this is an expression that returns a reference to a slice of u8, which indexed by an indexed value of p_field.
                    &[" ABCDEFG=#".as_bytes()[p_field[y * FIELD_WIDTH + x]]]
                );
    
                thread::sleep(time::Duration::from_millis(50));
            }
        }

        // draw current piece
        for px in 0..4 {
            for py in 0..4 {
                if tetromino[current_piece as usize].as_bytes()[rotate(px, py, current_rotation) as usize] == b'X' {
                    buffer.execute(cursor::MoveTo((current_y + px) as u16, (current_x + py) as u16))?;
                    buffer.write(&[current_piece + 65]);
                }
            }
        }
    }
    println!("Terminal size: {:?}", terminal::size()?);

    println!("{:?}", tetromino[0usize].chars().nth(0).unwrap());

    Ok(())
    
}

// remember to pass tetro and p_field!!!!
fn does_tetromino_fit (tetro: [&str; 7], p_field: [usize; FIELD_WIDTH & FIELD_HEIGHT], id_tetromino: u8, rotation: u8, x_pos: u8, y_pos: u8) -> bool {
    for px in 0..4 {
        for py in 0..4 {
            // gets index into piece
            let pi: u8 = rotate(px, py, rotation);

            // gets index into field
            let fi: u8 = (y_pos + py) * FIELD_WIDTH as u8 + (x_pos + px);
            
            if x_pos + px >= 0 && x_pos + px < FIELD_WIDTH as u8 {
                if y_pos + py >= 0 && y_pos + py < FIELD_HEIGHT as u8 {
                    if tetro[id_tetromino as usize].as_bytes()[pi as usize] == b'X' && p_field[fi as usize] != 0 {
                        return false;
                    }
                }
            }
        }
    } 
    
    true
}

fn rotate (px: u8, py: u8, r: u8) -> u8 {
    match r % 4 {
        0 => py * 4 + px,
        1 => 12 + py - (px * 4),
        2 => 15 - (py * 4) - px,
        3 => 3 - py + (px * 4),
        _ => 0,
    }
}
