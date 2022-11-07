mod board;

fn main() {
    println!("Hello World");
    let stuff = [[0u8; 8]; 8];
    let board1 = board::build_board(stuff);
    println!("{}", board1);

    let mut stuff2 = [[0u8; 8]; 8];
    stuff2 [0][0] = 1;
    let board2 = board::build_board(stuff2);
    println!("{}", board2);
}
