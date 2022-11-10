mod board;

fn main() {
    //_test_make_unmake_simple();
    //_test_make_unmake_castle();
    //_king_castles();
    _test_checks();
    _pawn_moves();
}

fn _pawn_moves() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.pawn_moves((4,4));
    println!("{:?}", moves);
    paint_pawn_moves(&mut board3, moves);

    board3.from_fen(starting_position);
    let moves2 = board3.pawn_moves((7,6));
    paint_pawn_moves(&mut board3, moves2);

    board3.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let moves3 = board3.pawn_moves((1,1));
    paint_pawn_moves(&mut board3, moves3);

    board3.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let moves4 = board3.pawn_moves((5,3));
    paint_pawn_moves(&mut board3, moves4);
}

fn _test_checks() {
    let stuff = [[0u8; 8]; 8];
    let a_pos = "8/8/3p4/5p2/8/8/2n5/1r2bq2 w - - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(a_pos);

    assert!(board3.in_check((0,0))==false, "a8 should not be in check");
    assert!(board3.in_check((0,1))==true, "b8 should be in check");
    assert!(board3.in_check((3,4))==false, "e5 should not be in check");
    assert!(board3.in_check((4,2))==true, "c4 should be in check");
    assert!(board3.in_check((4,3))==true, "d4 should be in check");
}

fn _king_castles() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.king_moves((4,4));
    println!("{:?}", moves);
    paint_moves(&mut board3, moves);

    board3.from_fen("8/r7/3P4/2K5/8/8/8/6B1 w - - 0 1");
    let moves2 = board3.king_moves((3,2));
    paint_moves(&mut board3, moves2);
}

fn _king_moves() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.king_moves((4,4));
    println!("{:?}", moves);
    paint_moves(&mut board3, moves);

    board3.from_fen("8/r7/3P4/2K5/8/8/8/6B1 w - - 0 1");
    let moves2 = board3.king_moves((3,2));
    paint_moves(&mut board3, moves2);
}

fn _queen_moves() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.queen_moves((4,4));
    println!("{:?}", moves);
    paint_moves(&mut board3, moves);

    board3.from_fen("8/r7/3P4/2Q5/8/8/8/6B1 w - - 0 1");
    let moves2 = board3.queen_moves((3,2));
    paint_moves(&mut board3, moves2);
}

fn _bishop_moves() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.bishop_moves((4,4));
    println!("{:?}", moves);
    paint_moves(&mut board3, moves);

    board3.from_fen("8/r7/3P4/2B5/8/8/8/6B1 w - - 0 1");
    let moves2 = board3.bishop_moves((3,2));
    paint_moves(&mut board3, moves2);
}

fn _rook_moves() {
    let stuff = [[0u8; 8]; 8];
    let starting_position = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
    let mut board3 = board::build_board(stuff);
    board3.from_fen(starting_position);

    let moves = board3.rook_moves((4,4));
    println!("{:?}", moves);
    paint_moves(&mut board3, moves);

    board3.from_fen("8/3b4/8/8/3R2Pb/3p4/8/8 w - - 0 1");
    let moves2 = board3.rook_moves((4,3));
    paint_moves(&mut board3, moves2);
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

    board3.from_fen("8/8/8/8/8/8/8/8 w KQkq - 0 1");
    let moves3 = board3.knight_moves((0,0));
    let moves4 = board3.knight_moves((1,0));
    let moves4 = board3.knight_moves((0,1));
    paint_moves(&mut board3, moves3);
}

fn paint_pawn_moves(board : &mut board::ChessBoard, moves : Vec<((usize, usize),(usize, usize, u8))>) {
    let mut a_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
    for amove in moves {
        a_vec.push((amove.0,(amove.1.0, amove.1.1)));
    }
    paint_moves(board, a_vec);
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
