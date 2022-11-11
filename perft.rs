mod board;

fn perft(board : &mut board::ChessBoard, perft_count :&mut u64, depth : u16) {
    let moves = board.get_all_moves();
    for amove in moves {
        // let captured_piece : u8 = board.make(amove.0, amove.1);
        let result = board.make(amove.0, amove.1);
        match result {
            None => {
                // Illegal move, already unmade
            },
            Some(captured_piece) => {
                if depth > 1 {
                    //print!("{:?}\n", amove);
                    // print!("{:?}\n", perft_count);
                    perft(board, perft_count, depth-1);
                } else {
                    *perft_count += 1;
                    // print!("{}\n", board);
                }
                board.unmake(amove.0, amove.1, captured_piece);
            }
        }
    }
}


fn main() {
    let mut perft_count = 0;

    let stuff = [[0u8; 8]; 8];
    let mut board = board::build_board(stuff);
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    board.from_fen(starting_position);
    perft(&mut board, &mut perft_count, 5);
    // perft(&mut board, &mut perft_count, 1);
    print!("{:?}\n", perft_count);
    print!("{}\n", board);
}
