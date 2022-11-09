use std::fmt;

use std::convert::TryFrom;

pub struct ChessBoard {
    pub board: [[u8; 8]; 8],
    opponent: u8,
    protagonist: u8,
}

pub fn build_board(board: [[u8; 8]; 8]) -> ChessBoard {
    ChessBoard{ board: board , opponent: BLACK , protagonist: WHITE }
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

    pub fn knight_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
        let mut dest_list = Vec::new();

        let mut rank = source.0;
        let mut file = source.1;

        if rank <= 6 && file <= 5 {
            dest_list.push((rank + 1, file + 2));
        }
        if rank <= 5 && file <= 6 {
            dest_list.push((rank + 2, file + 1));
        }
        if rank >= 1 && file <= 5 {
            dest_list.push((rank - 1, file + 2));
        }
        if rank >= 2 && file <= 5 {
            dest_list.push((rank - 2, file + 1));
        }
        if rank <= 6 && file >= 2 {
            dest_list.push((rank + 1, file - 2));
        }
        if rank <= 5 && file >= 1 {
            dest_list.push((rank + 2, file - 1));
        }
        if rank >= 1 && file >= 2 {
            dest_list.push((rank - 1, file - 2));
        }
        if rank >= 2 && file >= 1 {
            dest_list.push((rank - 2, file - 1));
        }
        for dest in dest_list {
            if self.board[dest.0][dest.1] != (clear_piece_color(self.board[dest.0][dest.1]) | self.protagonist) {
                move_vec.push((source, dest));
            }
        }
        move_vec
    }

    fn move_in_dir(& self, source: (usize, usize), move_vec : &mut Vec<((usize, usize),(usize, usize))>, lat_step : i8, hor_step : i8) {
        let mut rank = source.0;
        let mut file = source.1;
        while (hor_step != 1 || rank < 7) && (hor_step != -1 || rank > 0) && (lat_step != 1 || file < 7) && (lat_step != -1 || file > 0) {
            rank = step_usize(rank, hor_step);
            file = step_usize(file, lat_step);
            if self.board[rank][file] != 0b0000 {
                if self.protagonist == (self.board[rank][file] & WHITE) { // hit our piece
                    break;
                } else { // enemy piece
                    move_vec.push((source, (rank, file)));
                    break;
                }
            }
            move_vec.push((source, (rank, file)));
        }
    }

    pub fn rook_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
        self.move_in_dir(source, &mut move_vec, 1, 0);
        self.move_in_dir(source, &mut move_vec, 0, 1);
        self.move_in_dir(source, &mut move_vec, -1, 0);
        self.move_in_dir(source, &mut move_vec, 0, -1);
        move_vec
    }

    pub fn bishop_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
        self.move_in_dir(source, &mut move_vec, 1, 1);
        self.move_in_dir(source, &mut move_vec, -1, 1);
        self.move_in_dir(source, &mut move_vec, 1, -1);
        self.move_in_dir(source, &mut move_vec, -1, -1);
        move_vec
    }

    pub fn queen_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = self.rook_moves(source);
        move_vec.append(&mut self.bishop_moves(source));
        move_vec
    }

    pub fn king_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
        let mut dest_list = Vec::new();
        add_dest_if_on_board(source, &mut dest_list, 1, 1);
        add_dest_if_on_board(source, &mut dest_list, 1, -1);
        add_dest_if_on_board(source, &mut dest_list, 1, 0);
        add_dest_if_on_board(source, &mut dest_list, -1, 1);
        add_dest_if_on_board(source, &mut dest_list, -1, -1);
        add_dest_if_on_board(source, &mut dest_list, -1, 0);
        add_dest_if_on_board(source, &mut dest_list, 0, -1);
        add_dest_if_on_board(source, &mut dest_list, 0, 1);
        for dest in dest_list {
            if self.board[dest.0][dest.1] != (clear_piece_color(self.board[dest.0][dest.1]) | self.protagonist) {
                move_vec.push((source, dest));
            }
        }
        // TODO: add castling
        move_vec
    }


    pub fn make(&mut self, source: (usize, usize), dest: (usize, usize)) {
        if clear_piece_color(self.board[source.0][source.1]) == KING {
            if source.1.abs_diff(dest.1) > 1 { // Castling
                if source.1 > dest.1 { // Queenside
                    self.board[source.0][dest.1 + 1] = self.board[source.0][0];
                    self.board[source.0][0] = 0b0000;
                } else { // Kingside
                    self.board[source.0][dest.1 - 1] = self.board[source.0][7];
                    self.board[source.0][7] = 0b0000;
                }
            }
        }
        self.board[dest.0][dest.1] = self.board[source.0][source.1];
        self.board[source.0][source.1] = 0b0000;
    }

    pub fn unmake(&mut self, source: (usize, usize), dest: (usize, usize), captured_piece: u8) {
        if clear_piece_color(self.board[dest.0][dest.1]) == KING {
            if source.1.abs_diff(dest.1) > 1 { // UnCastling
                if source.1 > dest.1 { // Queenside
                    self.board[source.0][0] = self.board[source.0][dest.1 + 1];
                    self.board[source.0][dest.1 + 1] = 0b0000;
                } else { // Kingside
                    self.board[source.0][7] = self.board[source.0][dest.1 - 1];
                    self.board[source.0][dest.1 - 1] = 0b0000;
                }
            }
        }

        self.board[source.0][source.1] = self.board[dest.0][dest.1];
        self.board[dest.0][dest.1] = captured_piece;
    }

    pub fn unmake_ep(&mut self, source: (usize, usize), dest: (usize, usize), captured_piece: u8, ep: (usize, usize)) {
        self.board[ep.0][ep.1] = PAWN | self.opponent;
        self.unmake(source, dest, captured_piece);
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


fn step_usize(unsigned_int : usize, step : i8) -> usize {
    match step {
        0=>unsigned_int,
        1=>unsigned_int+1,
        -1=>unsigned_int-1,
        _=>panic!("Illegal step bigger than 1")
    }
}

fn add_dest_if_on_board (source : (usize, usize), dest_list : &mut Vec<(usize, usize)>, hor_step : i8, lat_step : i8){
    let rank = source.0;
    let file = source.1;
    if (hor_step != 1 || rank < 7) && (hor_step != -1 || rank > 0) && (lat_step != 1 || file < 7) && (lat_step != -1 || file > 0) {
        dest_list.push((step_usize(rank, lat_step), step_usize(file, hor_step)));
    }
}
