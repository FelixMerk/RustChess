mod board;

pub fn print_moves(moves :& Vec<((usize, usize), (usize, usize, u8))>) {
    for amove in moves {
        println!("{}, {}", board::square_to_alphanumeric(amove.0), board::square_to_alphanumeric((amove.1.0,amove.1.1)));
    }
}

fn perft(board : &mut board::ChessBoard, perft_count :&mut u64, depth : u16) {
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
                    perft(board, perft_count, depth-1);
                } else {
                    *perft_count += 1;
                    // print!("{}\n", board);
                    println!("{}, {}", board::square_to_alphanumeric(amove.0), board::square_to_alphanumeric((amove.1.0,amove.1.1)));
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
    if legal_moves == 0 {// check mate position
        *perft_count += 1;
        print!("Mate!");
        print!("{}\n", board);
    }
}


fn main() {
    let mut perft_count = 0;

    let stuff = [[0u8; 8]; 8];
    let mut board = board::build_board(stuff);

    // Expect 4,865,609	at depth 5
    // Actual 4865644
    // Actual 4865652
    // let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    // Expected 97862 at depth 3
    // Actual 97983
    // let starting_position = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ";
    let starting_position = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
    board.from_fen(starting_position);
    // perft(&mut board, &mut perft_count, 5);
    perft(&mut board, &mut perft_count, 1);
    print!("{:?}\n", perft_count);
    print!("{}\n", board);
}



