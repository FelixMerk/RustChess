mod board;

pub fn print_moves(moves :& Vec<((usize, usize), (usize, usize, u8))>) {
    for amove in moves {
        println!("{}, {}", board::square_to_alphanumeric(amove.0), board::square_to_alphanumeric((amove.1.0,amove.1.1)));
    }
}

fn perft(board : &mut board::ChessBoard, perft_count :&mut u64, depth : u16, debug_count :&mut u64) {
    let moves = board.get_all_moves();
    let mut legal_moves = 0;
    for amove in moves {
        // let captured_piece : u8 = board.make(amove.0, amove.1);
        let ep = board.ep;
        let white_kingside_castle = board.white_kingside_castle;
        let white_queenside_castle = board.white_queenside_castle;
        let black_kingside_castle = board.black_kingside_castle;
        let black_queenside_castle = board.black_queenside_castle;

        let result = board.make(amove.0, amove.1);
        match result {
            None => {
                // Illegal move, already unmade
            },
            Some(captured_piece) => {
                legal_moves += 1;
                if depth > 1 {
                    // print!("{:?}\n", amove);
                    // print!("{:?}\n", perft_count);
                    perft(board, perft_count, depth-1, debug_count);
                } else {
                    *perft_count += 1;
                    // if amove.1.1 != 0 { Promo
                    if (amove.0.1.abs_diff(amove.1.1) > 1) && (board.board[amove.1.0][amove.1.1] == 0b0001) { // Castled
                        *debug_count += 1;
                    }
                    // print!("{}\n", board);
                    // println!("{}, {}", board::square_to_alphanumeric(amove.0), board::square_to_alphanumeric((amove.1.0,amove.1.1)));
                }
                board.unmake(amove.0, amove.1, captured_piece);
            }
        }
        board.ep = ep;
        board.white_kingside_castle = white_kingside_castle;
        board.white_queenside_castle = white_queenside_castle;
        board.black_kingside_castle = black_kingside_castle;
        board.black_queenside_castle = black_queenside_castle;
    }
    if legal_moves == 0 {// check or stalemate position
        *perft_count += 1;
    }
}


fn main() {
    let mut perft_count = 0;
    let mut debug_count = 0;

    let stuff = [[0u8; 8]; 8];
    let mut board = board::build_board(stuff);

    /*
    let position_3 = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    board.from_fen(position_3);
    perft(&mut board, &mut perft_count, 4);
    assert!(perft_count == 43238, "Pos 3 Failed");
    perft_count = 0;

    let position_6 = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
    board.from_fen(position_6);
    perft(&mut board, &mut perft_count, 4);
    assert!(perft_count == 3894594, "Pos 6 Failed");
    perft_count = 0;
    */

    // Expect 4,865,609	at depth 5
    // Actual 4,865,644
    // Actual 4,865,652
    // Actual 4,865,617
    // let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    // Expected 4085603 at depth 4
    // Actual 4085604
    // let starting_position = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";

    // Expected 422333 at depth 4
    // Actual 422349
    let starting_position = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

    // Expected 2,103,487 at depth 4
    // Actual 2,103,531
    // let starting_position = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

    board.from_fen(starting_position);
    // perft(&mut board, &mut perft_count, 5);
    perft(&mut board, &mut perft_count, 4, &mut debug_count);
    print!("{:?}\n", perft_count);
    print!("{:?}\n", debug_count);
    print!("{}\n", board);
}



