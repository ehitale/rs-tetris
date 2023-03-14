use std::io;

fn main() {
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
    

    
}
