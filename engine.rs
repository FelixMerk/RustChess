mod board;
mod eval;

pub fn print_move(amove :& ((usize, usize), (usize, usize, u8))) {
    println!("{}, {}", board::square_to_alphanumeric(amove.0), board::square_to_alphanumeric((amove.1.0,amove.1.1)));
}

fn negamax(board : &mut board::ChessBoard, depth : u16) -> i32 {
    let moves = board.get_all_moves();
    let mut legal_moves = 0;
    let mut max : i32 = i32::MIN;
    for amove in moves {
        let ep = board.ep;
        let white_kingside_castle = board.white_kingside_castle;
        let white_queenside_castle = board.white_queenside_castle;
        let black_kingside_castle = board.black_kingside_castle;
        let black_queenside_castle = board.black_queenside_castle;

        let result = board.make(amove.0, amove.1);
        let score;
        match result {
            None => {
                // Illegal move, already unmade
            },
            Some(captured_piece) => {
                // print_move(&amove);
                //println!("{}", board.in_check(board.black_king_pos));
                //println!("{}", board);
                //println!("{:?}", board.black_king_pos);
                legal_moves += 1;
                if depth > 1 {
                    score = -1 * negamax(board, depth-1);
                } else {
                    score = -1 * eval::eval(board);
                }
                if score > max {
                    // println!("{}", score);
                    max = score;
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

    if legal_moves == 0  {// check or stalemate position
        if board.protagonist == board::WHITE {
            if board.in_check(board.white_king_pos) {
                max = -100000;
            } else { //stalemate
                max = 0;
            }
        } else {
            if board.in_check(board.black_king_pos) {
                max = -100000;
            } else { //stalemate
                max = 0;
            }
        }
    }

    return max;
}

fn main() {
    let stuff = [[0u8; 8]; 8];
    let mut board = board::build_board(stuff);

    /*
    // let position_1 = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = "7k/8/8/8/8/8/6P1/2QKRBN1 w - - 0 1";
    //board.from_fen(position_1);
    board.from_fen(pos);
    let score = negamax(&mut board, 4);
    print!("{}\n", score);
    */

    let mate_in_1 = "r3k2r/pp3p2/2n1p3/2pp1b2/6nq/1P1PPN2/PBP2PB1/R2QR1K1 b kq - 1 15";
    board.from_fen(mate_in_1);
    let score = negamax(&mut board, 2);
    println!("{}", score);

    /*
    let mate_in_3 = "6k1/pp3p1p/2p3p1/3p1P2/3P1KPP/4Q3/P1q5/8 w - - 0 26";
    board.from_fen(mate_in_3);
    let score = negamax(&mut board, 7);
    println!("{}", score);
    */

    /*
    let pos_force_stalemate = "4r2k/5Q2/8/8/8/8/2RR2P1/2RKRBN1 b - - 0 1";
    board.from_fen(pos_force_stalemate);
    let score = negamax(&mut board, 3);
    println!("{}", score);
    */
    
    /*
    let stale = "7k/5Q2/8/8/8/8/2RR2P1/2RKrBN1 w - - 0 2";
    board.from_fen(stale);
    let score = negamax(&mut board, 2);
    println!("{}", score);
    */

    /*
    let stale = "7k/5Q2/8/8/8/8/2RR2P1/2R1KBN1 b - - 0 2";
    board.from_fen(stale);
    let score = negamax(&mut board, 1);
    println!("{}", score);
    */
}
