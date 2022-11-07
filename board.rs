use std::fmt;

pub struct ChessBoard {
    board: [[u8; 8]; 8],
}

pub fn build_board(board: [[u8; 8]; 8]) -> ChessBoard {
    ChessBoard{ board: board }
}



impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..8 {
            for c in 0..8 {
                let piece = self.board[r][c];
                match piece{
                    0=>write!(f, "_").unwrap(),
                    _=>write!(f, "x").unwrap(),
                };
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}
