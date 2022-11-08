use std::fmt;
use std::convert::TryFrom;

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
                write!(f, "{}", piece_to_char(piece)).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}

impl ChessBoard {
    pub fn from_fen(&mut self, fen : &str)  {
        self.board = [[0u8; 8]; 8];
        let mut row = 0;
        let mut col = 0;
        for c in fen.chars() {
            if c != ' ' {
                let color : u8 = if c.is_lowercase() {
                    BLACK
                } else {
                    WHITE
                };
                match c.to_ascii_lowercase() {
                    '/'=> {
                        col = 0;
                        row += 1;
                    }
                    'p'=> {
                        self.board[row][col] = PAWN | color;
                        col += 1;
                    }
                    'r'=> {
                        self.board[row][col] = ROOK | color;
                        col += 1;
                    }
                    'n'=> {
                        self.board[row][col] = KNIGHT | color;
                        col += 1;
                    }
                    'b'=> {
                        self.board[row][col] = BISHOP | color;
                        col += 1;
                    }
                    'k'=> {
                        self.board[row][col] = KING | color;
                        col += 1;
                    }
                    'q'=> {
                        self.board[row][col] = QUEEN | color;
                        col += 1;
                    }
                    '1'..='8'=> {
                        col += usize::try_from(c.to_digit(10).unwrap()).unwrap();
                    }
                    _ => println!("Unsupported fen char"),
                };
            } else {
                break;
            }
        }
    }
}


const WHITE : u8 = 0b1000;
const BLACK : u8 = 0b0000;
const KING : u8 = 0b0001;
const QUEEN : u8 = 0b0010;
const PAWN : u8 = 0b0011;
const ROOK : u8 = 0b0100;
const KNIGHT : u8 = 0b0101;
const BISHOP : u8 = 0b0111;

pub fn piece_to_char(piece : u8) -> char {
    let piece_char : char = match clear_piece_color(piece) {
        KING=>'k',
        QUEEN=>'q',
        PAWN=>'p',
        ROOK=>'r',
        KNIGHT=>'n',
        BISHOP=>'b',
        0=>'_',
        6_u8 | 8_u8..=u8::MAX => 'X',
    };

    if (piece & WHITE) == WHITE {
        piece_char.to_ascii_uppercase()
    } else {
        piece_char
    }
}

pub fn clear_piece_color(piece : u8) -> u8 {
    piece & !WHITE
}


