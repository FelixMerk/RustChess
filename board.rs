use std::fmt;

use std::convert::TryFrom;
use std::convert::TryInto;

pub struct ChessBoard {
    pub board: [[u8; 8]; 8],
    opponent: u8,
    pub protagonist: u8,
    white_kingside_castle: bool,
    white_queenside_castle: bool,
    black_kingside_castle: bool,
    black_queenside_castle: bool,
    pub  ep: Option<(usize, usize)>,
    pub white_king_pos: (usize, usize),
    pub black_king_pos: (usize, usize),
}

pub fn build_board(board: [[u8; 8]; 8]) -> ChessBoard {
    ChessBoard{ board: board , opponent: BLACK , protagonist: WHITE, white_kingside_castle: true, white_queenside_castle: true, black_kingside_castle: true, black_queenside_castle: true, ep: None, white_king_pos: (7,4), black_king_pos: (0,4)}
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

        self.white_kingside_castle = false;
        self.white_queenside_castle = false;
        self.black_kingside_castle = false;
        self.black_queenside_castle = false;

        let mut row = 0;
        let mut col = 0;
        let mut space_count = 0;
        for c in fen.chars() {
            if c != ' ' && space_count == 0 {
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
                        if color == WHITE {
                            self.white_king_pos = (row, col);
                        } else {
                            self.black_king_pos = (row, col);
                        }
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
                if c == ' ' {
                    space_count += 1;
                } else if space_count == 1 { // To move
                    if c == 'w' {
                        self.protagonist = WHITE;
                    } else {
                        self.protagonist = BLACK;
                    }
                } else if space_count == 2 { // Castling Rights
                    if c == 'K' {
                        self.white_kingside_castle = true;
                    } else if c == 'Q' {
                        self.white_queenside_castle = true;
                    } else if c == 'k' {
                        self.black_kingside_castle = true;
                    } else if c == 'q' {
                        self.black_queenside_castle = true;
                    }
                } else if space_count == 3 { // EP information
                    if c == '-' {
                        self.ep = None
                    } else if c.is_ascii_digit() {
                        self.ep = Some(((8-c.to_digit(10).unwrap()).try_into().unwrap(), self.ep.unwrap().1));
                    } else {
                        self.ep = Some((2, letter_to_col(c)));
                    }
                }
            }
        }
    }

    pub fn knight_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
        let mut dest_list = Vec::new();

        let rank = source.0;
        let file = source.1;

        if rank <= 6 && file <= 5 {
            dest_list.push((rank + 1, file + 2));
        }
        if rank <= 5 && file <= 6 {
            dest_list.push((rank + 2, file + 1));
        }
        if rank >= 1 && file <= 5 {
            dest_list.push((rank - 1, file + 2));
        }
        if rank >= 2 && file <= 6 {
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
            let d_piece = self.board[dest.0][dest.1];
            if d_piece == 0b0000 || d_piece != (clear_piece_color(d_piece) | self.protagonist) {
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
            let d_piece = self.board[dest.0][dest.1];
            if d_piece == 0b0000 || d_piece != (clear_piece_color(d_piece) | self.protagonist) {
                move_vec.push((source, dest));
            }
        }
        // Castling
        if (self.white_kingside_castle && self.protagonist == WHITE) || (self.black_kingside_castle && self.protagonist == BLACK) {
            if !self.in_check(source) && !self.in_check((source.0, source.1 + 1)) && self.board[source.0][source.1+1] == 0 && self.board[source.0][source.1+2] == 0 {
                move_vec.push((source, (source.0, source.1 + 2)))
            }
        }
        if (self.white_queenside_castle && self.protagonist == WHITE) || (self.black_queenside_castle && self.protagonist == BLACK) {
            if !self.in_check(source) && !self.in_check((source.0, source.1 - 1)) && self.board[source.0][source.1-1] == 0 && self.board[source.0][source.1-2] == 0 && self.board[source.0][source.1-3] == 0 {
                move_vec.push((source, (source.0, source.1 - 2)))
            }
        }
        move_vec
    }

    pub fn in_check(& self, source: (usize, usize)) -> bool {
        // If square is being attacked

        // Knight Checks
        let knight_moves : Vec<((usize, usize),(usize, usize))> = self.knight_moves(source);
        for a_move in knight_moves {
            let dest = a_move.1;
            if self.board[dest.0][dest.1] == KNIGHT | self.opponent {
                return true;
            }
        }
        // Diagonal Checks
        let diagonal_moves : Vec<((usize, usize),(usize, usize))> = self.bishop_moves(source);
        for a_move in diagonal_moves {
            let dest = a_move.1;
            if self.board[dest.0][dest.1] == BISHOP | self.opponent {
                return true;
            }

            if self.board[dest.0][dest.1] == QUEEN | self.opponent {
                return true;
            }
        }
        // Horizontal/Vertical Checks
        let rook_moves : Vec<((usize, usize),(usize, usize))> = self.rook_moves(source);
        for a_move in rook_moves {
            let dest = a_move.1;
            if self.board[dest.0][dest.1] == ROOK | self.opponent {
                return true;
            }

            if self.board[dest.0][dest.1] == QUEEN | self.opponent {
                return true;
            }
        }

        // Pawn Moves
        if self.opponent == WHITE {
            if source.0 > 0 && source.1 > 0 {
                if self.board[source.0-1][source.1-1] == PAWN | self.opponent {
                    return true;
                }
            }
            if source.0 > 0 && source.1 < 7 {
                if self.board[source.0-1][source.1+1] == PAWN | self.opponent {
                    return true;
                }
            }
        }

        if self.opponent == BLACK {
            if source.0 < 7 && source.1 > 0 {
                if self.board[source.0+1][source.1-1] == PAWN | self.opponent {
                    return true;
                }
            }
            if source.0 < 7 && source.1 < 7 {
                if self.board[source.0+1][source.1+1] == PAWN | self.opponent {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn pawn_moves(& self, source: (usize, usize)) -> Vec<((usize, usize),(usize, usize, u8))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize, u8))> = Vec::new();
        if self.protagonist == BLACK {
            if source.0 < 6 {
                if self.board[source.0+1][source.1] == 0b0000 {
                    move_vec.push((source, (source.0+1, source.1, 0b0000)));
                }
                if source.1 < 7 && self.enemy_piece(source.0+1, source.1+1) {
                    move_vec.push((source, (source.0+1, source.1+1, 0b0000)));
                }
                if source.1 > 0 && self.enemy_piece(source.0+1,source.1-1) {
                    move_vec.push((source, (source.0+1, source.1-1, 0b0000)));
                }
                if self.ep.is_some() && source.1 < 7 && (source.0+1,source.1+1) == self.ep.unwrap() { // En Passent
                    move_vec.push((source, (source.0+1, source.1+1, 0b0000)));
                }
                if self.ep.is_some() && source.1 > 0 && (source.0+1,source.1-1) == self.ep.unwrap() { // En Passent
                    move_vec.push((source, (source.0+1, source.1-1, 0b0000)));
                }
            }
            if source.0 == 6 { // Promo
                if self.board[source.0+1][source.1] == 0b0000 {
                    move_vec.append(&mut create_promo_moves(source, (source.0+1, source.1)));
                }
                if source.1 < 7 && self.enemy_piece(source.0+1,source.1+1) {
                    move_vec.append(&mut create_promo_moves(source, (source.0+1, source.1+1)));
                }
                if source.1 > 0 && self.enemy_piece(source.0+1,source.1-1) {
                    move_vec.append(&mut create_promo_moves(source, (source.0+1, source.1-1)));
                }
            }
            if source.0 == 1 { // Move 2
                if self.board[source.0+1][source.1] == 0b0000 && self.board[source.0+2][source.1] == 0b0000 {
                    move_vec.push((source, (source.0+2, source.1, 0b0000)));
                }
            }
        } else {
            if source.0 > 1 {
                if self.board[source.0-1][source.1] == 0b0000 {
                    move_vec.push((source, (source.0-1, source.1, 0b0000)));
                }
                if source.1 < 7 && self.enemy_piece(source.0-1,source.1+1) {
                    move_vec.push((source, (source.0-1, source.1+1, 0b0000)));
                }
                if source.1 > 0 && self.enemy_piece(source.0-1,source.1-1) {
                    move_vec.push((source, (source.0-1, source.1-1, 0b0000)));
                }
                if self.ep.is_some() && source.1 < 7 && (source.0-1,source.1+1) == self.ep.unwrap() { // En Passent
                    move_vec.push((source, (source.0-1, source.1+1, 0b0000)));
                }
                if self.ep.is_some() && source.1 > 0 && (source.0-1,source.1-1) == self.ep.unwrap() { // En Passent
                    move_vec.push((source, (source.0-1, source.1-1, 0b0000)));
                }
            }
            if source.0 == 1 { // Promo
                if self.board[source.0-1][source.1] == 0b0000 {
                    move_vec.append(&mut create_promo_moves(source, (source.0-1, source.1)));
                }
                if source.1 < 7 && self.enemy_piece(source.0-1,source.1+1) {
                    move_vec.append(&mut create_promo_moves(source, (source.0-1, source.1+1)));
                }
                if source.1 > 0 && self.enemy_piece(source.0-1,source.1-1) {
                    move_vec.append(&mut create_promo_moves(source, (source.0-1, source.1-1)));
                }
            }
            if source.0 == 6 { // Move 2
                if self.board[source.0-1][source.1] == 0b0000 && self.board[source.0-2][source.1] == 0b0000 {
                    move_vec.push((source, (source.0-2, source.1, 0b0000)));
                }
            }
        }

        move_vec
    }

    fn enemy_piece(& self, row : usize, col : usize) -> bool {
        let piece = self.board[row][col];
        if piece != 0b0000 {
            return (piece & 0b1000) == self.opponent;
        }
        false
    }

    pub fn get_all_moves(&mut self) -> Vec<((usize, usize),(usize, usize, u8))> {
        let mut move_vec : Vec<((usize, usize),(usize, usize))> = Vec::new();
        let mut pawn_move_vec : Vec<((usize, usize),(usize, usize, u8))> = Vec::new();
        for row in 0..=7 {
            for col in 0..=7 {
                let piece : u8 = self.board[row][col];
                if (piece != 0b0000) && ((piece & 0b1000) == self.protagonist) {
                    let source = (row, col);
                    match clear_piece_color(piece) {
                        PAWN=>pawn_move_vec.append(&mut self.pawn_moves(source)),
                        KING=>move_vec.append(&mut self.king_moves(source)),
                        QUEEN=>move_vec.append(&mut self.queen_moves(source)),
                        ROOK=>move_vec.append(&mut self.rook_moves(source)),
                        KNIGHT=>move_vec.append(&mut self.knight_moves(source)),
                        BISHOP=>move_vec.append(&mut self.bishop_moves(source)),
                        0=>(),
                        6_u8 | 8_u8..=u8::MAX => assert!(false, "CHAOS"),
                    };
                }
            }
        }
        for amove in move_vec {
            pawn_move_vec.push((amove.0, (amove.1.0, amove.1.1, 0b0000)));
        }
        pawn_move_vec
    }

    pub fn make(&mut self, source: (usize, usize), dest: (usize, usize, u8)) -> Option<u8> {
        let captured_piece = self.board[dest.0][dest.1];
        let piece = self.board[source.0][source.1];

        if clear_piece_color(piece) == KING {
            if self.protagonist == WHITE { // Update King Pos
                self.white_king_pos = (dest.0, dest.1);
            } else {
                self.black_king_pos = (dest.0, dest.1);
            }
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
        if dest.2 == 0 {
            self.board[dest.0][dest.1] = piece;
        } else { // Promotion
            self.board[dest.0][dest.1] = dest.2 | self.protagonist;
        }
        self.board[source.0][source.1] = 0b0000;

        if piece == PAWN | self.protagonist {// Potential en passent
            match self.ep {
                Some(square) => {
                    if (dest.0, dest.1) == square {
                        // Taking en passent
                        // source row (3 or 4), dest col
                        self.board[source.0][dest.1] = 0b0000;
                    };
                },
                None => {},
            }
        }
        self.ep = None;

        if piece == PAWN | self.protagonist {// Future en passent
            if source.0.abs_diff(dest.0) == 2 {
                self.ep = Some(( (source.0 + dest.0) / 2,dest.1));
            }
        }

        let mut illegal = false;
        if ((self.protagonist == 0b1000) && self.in_check(self.white_king_pos)) || ((self.protagonist == 0b0000) && self.in_check(self.black_king_pos)) {
            // King illegally left in check
            illegal = true;
        }

        let temp = self.protagonist;
        self.protagonist = self.opponent;
        self.opponent = temp;

        if illegal {
            self.unmake(source, dest, captured_piece);
            return None;
        } else {
            return Some(captured_piece);
        }
    }

    pub fn unmake(&mut self, source: (usize, usize), dest: (usize, usize, u8), captured_piece: u8) {
        let temp = self.protagonist;
        self.protagonist = self.opponent;
        self.opponent = temp;

        if clear_piece_color(self.board[dest.0][dest.1]) == KING {
            if self.protagonist == WHITE {
                self.white_king_pos = source;
            } else {
                self.black_king_pos = source;
            }

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

        if dest.2 == 0 {
            self.board[source.0][source.1] = self.board[dest.0][dest.1];
        } else { // Unpromotion
            self.board[source.0][source.1] = PAWN | self.protagonist;
        }

        if self.board[source.0][source.1] == PAWN | self.protagonist {// Potential en passent
            if captured_piece == 0b0000 && source.1 != dest.1 {
                self.board[source.0][dest.1] = PAWN | self.opponent;
            }
        }

        self.board[dest.0][dest.1] = captured_piece;
    }

    pub fn unmake_ep(&mut self, source: (usize, usize), dest: (usize, usize, u8), captured_piece: u8, ep: (usize, usize)) {
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
    if (hor_step != 1 || file < 7) && (hor_step != -1 || file > 0) && (lat_step != 1 || rank < 7) && (lat_step != -1 || rank > 0) {
        dest_list.push((step_usize(rank, lat_step), step_usize(file, hor_step)));
    }
}

fn create_promo_moves(source: (usize, usize), dest: (usize, usize)) -> Vec<((usize, usize),(usize, usize, u8))> {
    let mut move_vec : Vec<((usize, usize),(usize, usize, u8))> = Vec::new();
    move_vec.push((source, (dest.0, dest.1, QUEEN)));
    move_vec.push((source, (dest.0, dest.1, ROOK)));
    move_vec.push((source, (dest.0, dest.1, BISHOP)));
    move_vec.push((source, (dest.0, dest.1, KNIGHT)));
    move_vec
}

fn letter_to_col(letter : char) -> usize {
    match letter {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => {
            assert!(false, "Bad Letter");
            0
        },
    }
}

pub fn square_to_alphanumeric(square: (usize, usize)) ->  String {
    let num = 8 - square.0;
    let letter = match square.1 {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => {
            assert!(false, "Bad number");
            "z"
        }
    };
    let num_as_string = num.to_string();
    // letter.push_str(&num_as_string);
    let together = format!("{}{}", letter, num_as_string);
    together
}
