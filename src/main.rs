//based off of Javidx9's tetris tutorial in C++. (https://youtu.be/8OK8_tHeCIA)
//no longer uses crossterm

use std::io::{Write, stdout};
// use crossterm::{ExecutableCommand, Result, terminal, event::{read, poll, Event, KeyCode}, cursor};
// use std::{thread, time};

mod experiment;

const FIELD_WIDTH: u8 = 12;
const FIELD_HEIGHT: u8 = 18;

fn main() {
    
    // let mut tetromino: [String; 7] = [
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from(), 
    //     String::from() 
    // ];
    

    // // leave this as a char (or u8), just convert to a byte it's time to write?
    let tetromino: [&str; 7] = [
        "..X...X...X...X.", 
        "..X..XX...X.....", 
        ".....XX..XX.....", 
        "..X..XX..X......", 
        ".X...XX...X.....", 
        ".X...X...XX.....", 
        "..X...X..XX....." 
    ];
    
    // let mut p_field: [u8; (FIELD_WIDTH * FIELD_HEIGHT) as usize] = [0; (FIELD_WIDTH * FIELD_HEIGHT) as usize];
    let p_field: &mut [u8] = &mut [0; (FIELD_WIDTH * FIELD_HEIGHT) as usize];

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


    let mut buffer = stdout();
    buffer.write(b"\x1B[2J").unwrap(); //clears the screen

    let mut current_piece = 0;
    let mut current_rotation = 1;
    let mut current_x = FIELD_WIDTH as u8 / 2;
    let mut current_y = 0;
    
    let game_over = false;

    while game_over == false {

        // // timing
        std::thread::sleep(std::time::Duration::from_millis(50));
        // // input
        // // logic 
        // // output

        //     // draw field

        for x in 0..FIELD_WIDTH {
            for y in 0..FIELD_HEIGHT {
                buffer.write(
                    format!("\x1B[{};{}H", x + 1, y + 1).as_bytes()    
                ).unwrap();
                buffer.write(&[
                    " ABCDEFG=#".as_bytes()[p_field[(y * FIELD_WIDTH + x) as usize] as usize]
                ]).unwrap();
            }
        }

            // draw current piece
        for px in 0..4 {
            for py in 0..4 {
                if tetromino[current_piece as usize].as_bytes()[rotate(px, py, current_rotation) as usize] == b'X' {
                    buffer.write(
                        format!("\x1B[{};{}H", current_x + px + 1, current_y + py + 1).as_bytes()
                    ).unwrap();
                    buffer.write(&[current_piece + 65]).unwrap();
                }
            }
        }
        
        buffer.flush().unwrap();
    } // end of game loop

    
}

// remember to pass tetro and p_field!!!!
fn _does_tetromino_fit (tetro: [&str; 7], p_field: [u8; (FIELD_WIDTH * FIELD_HEIGHT) as usize], id_tetromino: u8, rotation: u8, x_pos: u8, y_pos: u8) -> bool {
    for px in 0..4 {
        for py in 0..4 {
            // gets index into piece
            let pi: u8 = rotate(px, py, rotation);

            // gets index into field
            let fi: u8 = (y_pos + py) * FIELD_WIDTH as u8 + (x_pos + px);
            
            if x_pos + px > 0 && x_pos + px < FIELD_WIDTH as u8 {
                if y_pos + py > 0 && y_pos + py < FIELD_HEIGHT as u8 {
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
