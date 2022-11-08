mod board;

fn main() {
    println!("Hello World");
    let stuff = [[0u8; 8]; 8];
    let board1 = board::build_board(stuff);
    println!("{}", board1);

    let mut stuff2 = [[0u8; 8]; 8];
    stuff2 [0][0] = 1;
    stuff2 [0][4] = 4;
    stuff2 [5][4] = 9;
    let board2 = board::build_board(stuff2);
    println!("{}", board2);


    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);
    println!("{}", board3);
    let other_pos = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    board3.from_fen(other_pos);
    println!("{}", board3);
}
