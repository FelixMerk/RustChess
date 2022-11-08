mod board;

fn main() {
    //_test_make_unmake_simple();
    //_test_make_unmake_castle();
    _knight_moves();
}

fn _knight_moves() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.knight_moves((4,4));
    println!("{:?}", moves);
    paint_moves(&mut board3, moves);

    board3.from_fen("8/5p2/2b5/4N3/2K5/5P2/8/8 w - - 0 1");
    let moves2 = board3.knight_moves((3,4));
    println!("{}", board3);
    paint_moves(&mut board3, moves2);
}

fn paint_moves(board : &mut board::ChessBoard, moves : Vec<((usize, usize),(usize, usize))>) {
    for amove in moves {
        board.board[amove.1.0][amove.1.1] = 0b11111;
    }
    println!("{}", board);
}

fn _test_basic_boards() {
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
}

fn _test_make_unmake_simple() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);
    println!("Starting Position");
    println!("{}", board3);

    board3.make((6,4), (4,4));
    println!("1. e4");
    println!("{}", board3);

    board3.unmake((6,4), (4,4), 0b0000);
    println!("unmake e4");
    println!("{}", board3);
}

fn _test_make_unmake_castle() {
    let debug = false;
    let stuff = [[0u8; 8]; 8];
    let castle_pos = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(castle_pos);

    let first_pos = board3.board;

    if debug {
        println!("Castle Pos");
        println!("{}", board3);
    }

    board3.make((7,4), (7,6));
    if debug {
        println!("1. O-O");
        println!("{}", board3);
    }

    board3.unmake((7,4), (7,6), 0b0000);
    if debug {
        println!("unmake e4");
        println!("{}", board3);
    }
    assert!(board3.board == first_pos);

    board3.make((7,4), (7,2));
    println!("{}", board3);
    board3.unmake((7,4), (7,2), 0b0000);
    println!("{}", board3);

    board3.make((0,4), (0,6));
    println!("{}", board3);
    board3.unmake((0,4), (0,6), 0b0000);
    println!("{}", board3);

    board3.make((0,4), (0,2));
    println!("{}", board3);
    board3.unmake((0,4), (0,2), 0b0000);
    println!("{}", board3);
}

fn _test_fen() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);
    println!("Starting Position");
    println!("{}", board3);

    let other_pos = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
    board3.from_fen(other_pos);
    println!("Test Position");
    println!("{}", board3);
}
