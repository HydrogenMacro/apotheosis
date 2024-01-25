use std::{
    collections::{
        HashSet,
        HashMap
    },
    cmp,
    default::Default,
    fmt
};
use ethnum::*;
use nohash_hasher::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Direction(i8, i8);
impl Direction {
    pub const fn dx(&self) -> i8 {
        return self.0;
    }
    pub const fn dy(&self) -> i8 {
        return self.1;
    }
    pub const fn as_square_pos_delta(&self) -> u8 {
        return (self.0 + self.1 * 8) as u8;
    }
    pub const N: Direction = Direction(0, 1);
    pub const NE: Direction = Direction(1, 1);
    pub const E: Direction = Direction(1, 0);
    pub const SE: Direction = Direction(1, -1);
    pub const S: Direction = Direction(0, -1);
    pub const SW: Direction = Direction(-1, -1);
    pub const W: Direction = Direction(-1, 0);
    pub const NW: Direction = Direction(-1, 1);
    
    pub const CARDINALS: [Direction; 4] = [Direction::N, Direction::E, Direction::S, Direction::W];
    pub const ORDINALS: [Direction; 4] = [Direction::NE, Direction::SE, Direction::SW, Direction::NW];
    pub const COMPASS_ROSE: [Direction; 8] = [Direction::N, Direction::NE, Direction::E, Direction::SE, Direction::S, Direction::SW, Direction::W, Direction::NW];
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BoardSquare(pub u8);
impl BoardSquare {
    pub const fn pos(&self) -> u8 {
        return self.0;
    }
    pub fn from(board_square_notation: &str) -> BoardSquare {
        if board_square_notation.len() != 2 {
            panic!("BoardSquare::from takes a 2 lengthed string, like \"d3\"");
        }
        if let [col, row] = board_square_notation.as_bytes() {
            let col_value: u8 = match col {
                b'a' => 0,
                b'b' => 1,
                b'c' => 2,
                b'd' => 3,
                b'e' => 4,
                b'f' => 5,
                b'g' => 6,
                b'h' => 7,
                _ => unreachable!()
            };
            let row_value: u8 = match row {
                b'8' => 0,
                b'7' => 8,
                b'6' => 16,
                b'5' => 24,
                b'4' => 32,
                b'3' => 40,
                b'2' => 48,
                b'1' => 56,
                _ => unreachable!()
            };
            return BoardSquare(row_value + col_value);
        }
        panic!("BoardSquare::from() failed for {}", board_square_notation);
    }
    pub const fn x(&self) -> u8 {
        return self.0 & 7;
    }
    pub const fn y(&self) -> u8 {
        return self.0 >> 3;
    }
    pub const fn get_square_in_direction(&self, dir: &Direction) -> Option<BoardSquare> {
        let new_x = self.x() as i8 + dir.dx();
        let new_y = self.y() as i8 + dir.dy();
        if new_x < 0 
        || new_x >= 8
        || new_y < 0 
        || new_y >= 8 {
            return None;
        }
        return Some(BoardSquare((new_x + new_y * 8) as u8));
    }
    pub fn get_all_squares_in_direction(&self, dir: &Direction) -> Vec<BoardSquare> {
        let amount_of_squares = match *dir {
            Direction::N => self.y(), 
            Direction::NE => cmp::min(7 - self.x(), self.y()),
            Direction::E => 7 - self.x(), 
            Direction::SE => 7 - cmp::max(self.x(), self.y()),
            Direction::S => 7 - self.y(), 
            Direction::SW => 7 - cmp::max(7 - self.x(), self.y()),
            Direction::W => self.x(), 
            Direction::NW => cmp::min(self.x(), self.y()),
            _ => panic!("get_all_squares_in_direction only supports cardinal/ordinal directions")
        };
        let mut squares_in_direction = Vec::with_capacity(amount_of_squares as usize);
        for square_num in 1..=amount_of_squares {
            let square_in_dir = (self.pos() as i8 + ((dir.dx() * square_num as i8) + (dir.dy() * square_num as i8 * -8))) as u8;
            squares_in_direction.push(BoardSquare(square_in_dir));
        }
        return squares_in_direction;
    }
    
    pub fn distance_from(&self, target_square: &BoardSquare) -> u8 {
        // this function should be const; however, cmp::max isn't
        return cmp::max((self.x() as i8 - target_square.x() as i8).abs(), (self.y() as i8 - target_square.y() as i8).abs()) as u8;
    }
}
impl fmt::Display for BoardSquare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row_chars = "abcdefgh".chars();
        return write!(
            f,
            "{}{}", 
            row_chars.nth(self.x() as usize).unwrap(), 
            8 - self.y()
        );
    }
}

pub type BoardColor = u8;
pub const BLACK: BoardColor = 0b0000;
pub const WHITE: BoardColor = 0b0001;

pub type BoardPieceType = u8;
pub const PAWN: BoardPieceType = 0b0010;
pub const KNIGHT: BoardPieceType = 0b0100;
pub const BISHOP: BoardPieceType = 0b0110;
pub const ROOK: BoardPieceType = 0b1000;
pub const QUEEN: BoardPieceType = 0b1010;
pub const KING: BoardPieceType = 0b1100;

pub type BoardPiece = u8;
pub const fn is_piece(possible_piece: BoardPiece) -> bool {
    return possible_piece & 0b1111 != 0;
}
pub const fn get_piece_type(piece: BoardPiece) -> BoardPieceType {
    return piece & 0b1110;
}
pub const fn get_piece_color(piece: BoardPiece) -> BoardPieceType {
    return piece & 0b0001;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BoardMove(u16);

impl BoardMove {
    pub const CASTLE_BQ: BoardMove = BoardMove(0b1000_0000_0000_0000u16);
    pub const CASTLE_BK: BoardMove = BoardMove(0b1010_0000_0000_0000u16);
    pub const CASTLE_WQ: BoardMove = BoardMove(0b1100_0000_0000_0000u16);
    pub const CASTLE_WK: BoardMove = BoardMove(0b1110_0000_0000_0000u16);
    
    pub fn new(origin_square: &BoardSquare, dest_square: &BoardSquare) -> BoardMove {
        return BoardMove(
                ((origin_square.pos() as u16) << 9) 
                | ((dest_square.pos() as u16) << 3)
        );
    }
    pub fn new_from_square_positions(origin_square_pos: u8, dest_square_pos: u8) -> BoardMove {
        return BoardMove(
                ((origin_square_pos as u16) << 9)
                | ((dest_square_pos as u16) << 3)
        );
    }
    pub const EN_PASSANT_MASK: BoardMove = 0b0000_0000_0000_0100u16;
    pub fn as_en_passant(&self) -> BoardSquare {
        return BoardMove(self.0 | BoardMove::EN_PASSANT_MASK);
    }
    pub const fn is_en_passant(&self) -> bool {
        return (self.0 & BoardMove::EN_PASSANT_MASK) != 0;
    }
    pub const fn is_castle(&self) -> bool {
        return (self.0 & 1) == 1;
    }
    pub const fn from_square(&self) -> BoardSquare {
        return BoardSquare(((self.0 >> 9) & 0b111111) as u8);
    }
    pub const fn dest_square(&self) -> BoardSquare {
        return BoardSquare(((self.0 >> 3) & 0b111111) as u8);
    }
}
impl fmt::Display for BoardMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{} => {}",
            self.from_square(),
            self.dest_square()
        );
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct BoardPieces {
    pawns: [Vec<BoardSquare>; 2],
    knights: [Vec<BoardSquare>; 2],
    bishops: [Vec<BoardSquare>; 2],
    rooks: [Vec<BoardSquare>; 2],
    queens: [Vec<BoardSquare>; 2],
    kings: [Option<BoardSquare>; 2]
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct BoardSquareInfo(Option<BoardPiece>, [u32; 2]);
impl BoardSquareInfo {
    pub fn occupant(&self) -> Option<BoardPiece> { self.0 }
    pub fn visibility(&self) -> [u32; 2] { self.1 }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BoardInfo {
    valid_moves: [Vec<BoardMove>; 2],
    board_pieces: BoardPieces,
    square_control: [BoardSquareInfo; 64]
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Board(u256, u32);
impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut fen_parts = fen.split_whitespace();
        
        let fen_board = fen_parts.next().unwrap();
        let mut board_image = U256::new(0);
        let mut board_state = 0u32;

        let mut current_board_image_pos = 0u8;
        
        for fen_board_char in fen_board.chars() {
            let possible_board_piece = match fen_board_char {
                'p' => Some(BLACK | PAWN),
                'n' => Some(BLACK | KNIGHT),
                'b' => Some(BLACK | BISHOP),
                'r' => Some(BLACK | ROOK),
                'q' => Some(BLACK | QUEEN),
                'k' => Some(BLACK | KING),
                'P' => Some(WHITE | PAWN),
                'N' => Some(WHITE | KNIGHT),
                'B' => Some(WHITE | BISHOP),
                'R' => Some(WHITE | ROOK),
                'Q' => Some(WHITE | QUEEN),
                'K' => Some(WHITE | KING),
                '1' => { current_board_image_pos += 1; None },
                '2' => { current_board_image_pos += 2; None },
                '3' => { current_board_image_pos += 3; None },
                '4' => { current_board_image_pos += 4; None },
                '5' => { current_board_image_pos += 5; None },
                '6' => { current_board_image_pos += 6; None },
                '7' => { current_board_image_pos += 7; None },
                '8' => { current_board_image_pos += 8; None },
                _ => None
            };
            if let Some(board_piece) = possible_board_piece {
                board_image |= U256::from(board_piece) << U256::from(current_board_image_pos as u32 * 4);
                current_board_image_pos += 1;
            }
        }
        assert_eq!(current_board_image_pos, 64);

        let active_color = fen_parts.next().unwrap();
        let active_color_bit_flag_mask = if active_color == "w" { 1u32 << 31u32 } else { 0u32 };
        board_state |= active_color_bit_flag_mask;
        
        let castle_availibility = fen_parts.next().unwrap();
        for castle_flag in castle_availibility.chars() {
            let castle_flag_mask = match castle_flag {
                'k' => 1u32 << 30,
                'q' => 1u32 << 29,
                'K' => 1u32 << 28,
                'Q' => 1u32 << 27,
                '-' | _ => 0
            };
            board_state |= castle_flag_mask;
        }
        
        let en_passant_target = fen_parts.next().unwrap();
        if en_passant_target != "-" {
            let en_passant_target_square = BoardSquare::from(en_passant_target);
            board_state |= 1u32 << 26;
            board_state |= (en_passant_target_square.pos() as u32) << 20;
        }

        let board = Board(board_image, board_state);
        return board;
    }
    pub const fn active_color(&self) -> BoardColor {
        let mask = 1u32;
        return ((self.1 >> 31) & mask) as u8;
    }
    pub const fn castle_availibility(&self) -> [(bool, bool); 2] {
        let mask = 0b1111u32;
        let bitflags = (self.1 >> 27) & mask;
        return [
            (
                bitflags & 0b1000 != 0,
                bitflags & 0b0100 != 0
            ),
            (
                bitflags & 0b0010 != 0,
                bitflags & 0b0001 != 0
            )
        ];
    }
    pub fn en_passant_target_square(&self) -> Option<BoardSquare> {
        let flag_mask = 1u32;
        let target_square_mask = 0b111111u32;
        let target_square_exists = ((self.1 >> 26) & flag_mask) != 0;
        if !target_square_exists {
            return None;
        }
        let target_square = BoardSquare(((self.1 >> 20) & target_square_mask) as u8);
        return Some(target_square);
    }
    pub fn get_piece_at(&self, square: &BoardSquare) -> Option<BoardPiece> {
        let mask_distance_away = U256::from(square.pos()) * 4;
        let mask = U256::new(0b1111);
        let square_contents = ((self.0 >> mask_distance_away) & mask).as_u8();
        if !is_piece(square_contents) {
            return None;
        }
        return Some(square_contents);
    }
    pub fn get_pieces(&self) -> BoardPieces {
        let mut board_pieces: BoardPieces = Default::default();
        for i in 0..64 {
            let piece_square = BoardSquare(i);
            if let Some(piece) = self.get_piece_at(&piece_square) {
                let piece_color = get_piece_color(piece);
                match get_piece_type(piece) {
                    PAWN => board_pieces.pawns[piece_color].push(piece_square),
                    KNIGHT => board_pieces.knights[piece_color].push(piece_square),
                    BISHOP => board_pieces.bishops[piece_color].push(piece_square),
                    ROOK => board_pieces.rooks[piece_color].push(piece_square),
                    QUEEN => board_pieces.queens[piece_color].push(piece_square),
                    KING => board_pieces.kings[piece_color] = Some(piece_square),
                    _ => {},
                }
            }
        }
        return board_pieces;
    }
    pub fn get_board_info(&self) -> BoardInfo {
        let mut valid_moves: [Vec<BoardMove>; 2] = [Vec::new(), Vec::new()];
        let mut board_pieces = self.get_pieces();
        let mut square_control: [BoardSquareInfo; 64] = Default::defualt();

        let mut pinned_pieces: [IntMap<u8, Direction>; 2] = [IntMap::new(), IntMap::new()];
        for king_color in [BLACK, WHITE] {
            if let Some(square_of_king) = board_pieces.kings[king_color] {
                for (
                    pinner_piece_type,
                    pinner_piece_move_directions
                ) in [
                    (ROOK, Direction::CARDINALS),
                    (BISHOP, Direction::ORDINALS)
                ] {
                    for dir in pinner_piece_move_directions {
                        // pinned piece is of the same color as color being checked
                        let mut possible_pinned_piece: Option<u8> = None;
                        'pin_direction_scan: for square_in_dir in square_of_king.get_all_squares_in_direction(dir) {
                            if let Some(piece_in_dir) = self.get_piece_at(&square_in_dir) {
                                let piece_in_dir_color = get_piece_color(piece_in_dir);
                                if let Some(pinned_piece) = possible_pinned_piece {
                                    let piece_in_dir_type = get_piece_type(piece_in_dir);
                                    if piece_in_dir_type == pinner_piece_type || piece_in_dir_type == QUEEN {
                                        if piece_in_dir_color != king_color {
                                            pinned_pieces[king_color].insert(square_in_dir.pos(), dir);

                                        }
                                    }
                                    break 'pin_direction_scan;
                                } else {
                                    // no pinned piece
                                    if piece_in_dir_color == king_color {
                                        possible_pinned_piece = Some(piece_in_dir);
                                    } else {
                                        // maybe store discovered check possibilities
                                        break 'pin_direction_scan;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        for origin_square_pos in 0..64 {
            let origin_square = BoardSquare(origin_square_pos);
            let possible_origin_piece = self.get_piece_at(&origin_square);
            if let None = possible_origin_piece { continue; }
            let origin_piece = possible_origin_piece.unwrap();
            
            let origin_piece_color = get_piece_color(origin_piece);
            let is_white = origin_piece_color == WHITE;
            let origin_piece_type = get_piece_type(origin_piece);
            
            match origin_piece_type {
                PAWN => {
                    if let Some(pinned_direction) = pinned_pieces.get(&origin_piece_square_pos) {
                        continue;
                    }
                    let dir = if is_white { -1i8 } else { 1i8 };
                    let base_reachable_square = origin_square
                        .get_square_in_direction(&Direction(0, dir * 1))
                        .expect("this can only be invalid in invalid positions");
                    if let None = self.get_piece_at(&base_reachable_square) {
                        valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &base_reachable_square));
                    }
                    
                    let is_on_home_square = origin_square.y() == 6 && is_white 
                        || origin_square.y() == 1 && !is_white;
                    if is_on_home_square {
                        let extended_reachable_square = origin_square
                            .get_square_in_direction(&Direction(0, dir * 2))
                            .expect("cannot go oob when on home square");
                        if let None = self.get_piece_at(&extended_reachable_square) {
                            valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &extended_reachable_square));
                        }
                    }
                    
                    let possible_capturable_directions = [
                        Direction(-1, dir * 1),
                        Direction(1, dir * 1)
                    ];
                    for possile_capturable_direction in possible_capturable_directions {
                        let possible_capturable_square = self.get_square_in_direction(&possible_capturable_direction);

                        if let Some(capturable_square) = possible_capturable_square {
                            // en passant
                            if let Some(en_passant_target_square) = self.en_passant_target_square() {
                                if en_passant_target_square == capturable_square {
                                    let en_passant_captured_square = self.get_square_in_direction(&Direction(possible_capturable_direction.dx(), 0));
                                    let en_passant_captured_piece = self.get_piece_at(en_passant_captured_square);
                                    if origin_piece_color != get_color_of(en_passant_captured_piece) {
                                        valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &capturable_square).as_en_passant());
                                        continue;
                                    }
                                }
                            }
                            if let Some(capturable_piece) = self.get_piece_at(&capturable_square) {
                                let capturable_piece_color = get_piece_color(capturable_piece);
                                if origin_piece_color != capturable_piece_color {
                                    valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &capturable_square));
                                }
                            }
                        }
                    }
                },
                KNIGHT | KING => {
                    if let Some(pinned_direction) = pinned_pieces.get(&origin_piece_square_pos) {
                        // kings cannot be pinned, and knights cannot move when pinned
                        continue;
                    }
                    let move_directions = match origin_piece_type {
                        KNIGHT => [
                            Direction(1, 2),
                            Direction(-1, 2),
                            Direction(1, -2),
                            Direction(-1, -2),
                            Direction(2, 1),
                            Direction(-2, 1),
                            Direction(2, -1),
                            Direction(-2, -1)
                        ],
                        KING => Direction::COMPASS_ROSE,
                        _ => unreachable!()
                    };
                    for move_direction in move_directions.into_iter() {
                        let possible_reachable_square = origin_square.get_square_in_direction(
                            move_direction
                        );
                        if let Some(reachable_square) = possible_reachable_square {
                            if let Some(reachable_piece) = self.get_piece_at(&reachable_square) {
                                if get_piece_color(reachable_piece) != origin_piece_color {
                                    valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &reachable_square));
                                }
                            } else {
                                valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &reachable_square));
                            }
                        }
                    }
                },
                BISHOP | ROOK | QUEEN => {
                    let move_directions = if let Some(pinned_direction) = pinned_pieces.get(&origin_piece_square_pos) {
                        let pinned_directions = &[pinned_direction, Direction(-pinned_direction.dx(), -pinned_direction.dy())][..];
                        match origin_piece_type {
                            BISHOP => if &Direction::ORDINALS[..].contains(pinned_direction) { pinned_direction } else { &[] },
                            ROOK => if &Direction::CARDINALS[..].contains(pinned_direction) { pinned_direction } else { &[] },
                            QUEEN => pinned_directions,
                            _ => unreachable!()
                        }
                    } else {
                        match origin_piece_type {
                            BISHOP => &Direction::ORDINALS[..],
                            ROOK => &Direction::CARDINALS[..],
                            QUEEN => &Direction::COMPASS_ROSE[..],
                            _ => unreachable!()
                        }
                    };
                    for move_direction in move_directions.into_iter() {
                        let reachable_squares = origin_square.get_all_squares_in_direction(move_direction);
                        let mut can_still_move = true;
                        let mut seen_pieces: Vec<BoardPiece> = Vec::new();
                        for reachable_square in reachable_squares.into_iter() {
                            if can_still_move {
                                if let Some(reachable_piece) = self.get_piece_at(&reachable_square) {
                                    if get_piece_color(reachable_piece) != origin_piece_color {
                                        valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &reachable_square));
                                    }
                                    can_still_move = false;
                                } else {
                                    valid_moves[origin_piece_color].push(BoardMove::new(&origin_square, &reachable_square));
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }

        return BoardInfo {
            valid_moves,
            board_pieces,
            square_control
        };
    }
    pub fn create_board_from_move(&self, board_move: &BoardMove) -> Board {
        let mut new_board = self.clone();
        if board_move.is_castle() {
            let [castle_color, castle_side] = match *board_move {
                BoardMove::CASTLE_BQ => [1, 1],
                
                _ => unreachable!()
            };
            todo!();
        }
        // is not castle
        let from_square_pos = board_move.from_square().pos();
        let dest_square_pos = board_move.dest_square().pos();
        let mask = U256::new(0b1111);
        let from_piece = ((self.0 >> U256::from(from_square_pos * 4)) & mask).as_u8();
        let _dest_piece = ((self.0 >> U256::from(from_square_pos * 4)) & mask).as_u8();
        
        // clear from_square and dest_square
        new_board.0 &= !(mask << U256::from(from_square_pos * 4));
        new_board.0 &= !(mask << U256::from(dest_square_pos * 4));
        
        // set dest_square
        new_board.0 |= U256::from(from_piece << dest_square_pos * 4);
        return new_board;
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity(64 + 8);
        let mask = U256::new(0b1111);
        for row in (0..64).step_by(8) {
            for col in 0..8 {
                let board_square = U256::new((row + col) * 4);
                let board_piece = ((self.0 >> board_square) & mask).as_u8();
                let board_piece_char = match board_piece {
                    _ if board_piece == BLACK | PAWN => 'p',
                    _ if board_piece == BLACK | KNIGHT => 'n',
                    _ if board_piece == BLACK | BISHOP => 'b',
                    _ if board_piece == BLACK | ROOK => 'r',
                    _ if board_piece == BLACK | QUEEN => 'q',
                    _ if board_piece == BLACK | KING => 'k',
                    _ if board_piece == WHITE | PAWN => 'P',
                    _ if board_piece == WHITE | KNIGHT => 'N',
                    _ if board_piece == WHITE | BISHOP => 'B',
                    _ if board_piece == WHITE | ROOK => 'R',
                    _ if board_piece == WHITE | QUEEN => 'Q',
                    _ if board_piece == WHITE | KING => 'K',
                    _ => ' '
                };
                s.push(board_piece_char);
            }
            s.push('\n');
        }
        return write!(f, "{}", s);
    }
}

