use board;

fn piece_value(piece : u8) -> i32 {
    match board::clear_piece_color(piece) {
        board::PAWN => 1,
        board::BISHOP => 3,
        board::KNIGHT => 3,
        board::ROOK => 5,
        board::QUEEN => 9,
        board::KING => 0,
        0_u8 => 0, // empty square
        6_u8 | 8_u8..=u8::MAX => todo!(),
    }
}

pub fn eval(board : &mut board::ChessBoard) -> i32 {
    let mut score : i32 = 0;
    for row in 0..=7 {
        for col in 0..=7 {
            let piece = board.board[row][col];
            let val = piece_value(piece);
            if piece & 0b1000 == board::WHITE {
                score += val;
            } else {
                score -= val;
            }
        }
    }
    if board.protagonist == board::WHITE {
        score = score;
    } else {
        score = -1*score;
    }
    score
}
