use std::{
    collections::HashSet,
    cmp,
    default::Default
};
use ethnum::*;

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Direction(i8, i8);
impl Direction {
    #[inline]
    pub const fn dx(&self) {
        return self.0;
    }
    #[inline]
    pub const fn dy(&self) {
        return self.1;
    }
    pub const N: Direction = Direction(0, 1); 
    pub const NE: Direction = Direction(1, 1); 
    pub const E: Direction = Direction(1, 0); 
    pub const SE: Direction = Direction(1, -1); 
    pub const S: Direction = Direction(0, -1); 
    pub const SW: Direction = Direction(-1, -1); 
    pub const W: Direction = Direction(-1, 0); 
    pub const NW: Direction = Direction(-1, 1); 
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct BoardSquare(u8);
impl BoardSquare {
    #[inline]
    pub const fn pos(&self) -> u8 {
        return self.0;
    }
    pub const fn from(board_square_notation: &str) -> BoardSquare {
        if board_square_notation.len() != 2 {
            panic!("BoardSquare::from takes a 2 lengthed string, like \"d3\"");
        }
        if let [row, col] = board_square_notation.as_bytes() {
            let row_value: u8 = match row {
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
            let col_value: u8 = match col {
                b'a' => 0,
                b'b' => 8,
                b'c' => 16,
                b'd' => 24,
                b'e' => 32,
                b'f' => 40,
                b'g' => 48,
                b'h' => 56,
                _ => unreachable!()
            };
            return BoardSquare(row_value + col_value);
        }
        panic!("BoardSquare::from() failed");
    }
    #[inline]
    pub const fn x(&self) -> u8 {
        return self.0 & 7;
    }
    #[inline]
    pub const fn y(&self) -> u8 {
        return self.0 << 3;
    }
    pub const fn get_square_in_direction(&self, dir: Direction) -> Option<BoardSquare> {
        let new_x = self.x() + dir.dx();
        let new_y = self.y() + dir.dy();
        if new_x < 0 
        || new_x >= 8
        || new_y < 0 
        || new_y >= 8 {
            return None;
        }
        return Some(BoardSquare(new_x + new_y * 8));
    }
}

pub type Color = u8;
pub const BLACK: Color = 0b0000;
pub const WHITE: Color = 0b0001;

pub type PieceType = u8;
pub const PAWN: PieceType = 0b0010;
pub const KNIGHT: PieceType = 0b0100;
pub const BISHOP: PieceType = 0b0110;
pub const ROOK: PieceType = 0b1000;
pub const QUEEN: PieceType = 0b1010;
pub const KING: PieceType = 0b1100;

pub type Piece = u8;
#[inline]
pub const fn is_piece(possible_piece: Piece) -> bool {
    return possible_piece & 0b1111 != 0;
}
#[inline]
pub const fn get_piece_type(piece: Piece) -> PieceType {
    return piece & 0b1110;
}
#[inline]
pub const fn get_piece_color(piece: Piece) -> PieceType {
    return piece & 0b0001;
}

#[derive(PartialEq, Debug)]
pub struct BoardMove(u16);

impl BoardMove {
    pub const CASTLE_BQ: BoardMove = BoardMove(0b1000_0000_0000_0000u16);
    pub const CASTLE_BK: BoardMove = BoardMove(0b1010_0000_0000_0000u16);
    pub const CASTLE_WQ: BoardMove = BoardMove(0b1100_0000_0000_0000u16);
    pub const CASTLE_WK: BoardMove = BoardMove(0b1110_0000_0000_0000u16);
    
    #[inline]
    pub fn from_board_squares(origin_square: BoardSquare, dest_square: BoardSquare) -> BoardMove {
        return BoardMove(
                ((origin_square.pos() as u16) << 9) 
                | ((dest_square.pos() as u16) << 3)
        );
    }
    #[inline]
    pub fn from_board_squares_as_en_passant(origin_square: BoardSquare, dest_square: BoardSquare) -> BoardMove {
        return BoardMove(
            0b0000_0000_0000_0100u16
                | ((origin_square.pos() as u16) << 9)
                | ((dest_square.pos() as u16) << 3)
        );
        // maybe just provide a mask
    }
}

#[derive(PartialEq, Debug)]
pub struct Board(u256, u32);
impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut fen_parts = fen.split_whitespace();
        
        let fen_board = fen_parts.next().unwrap();
        let mut board_image = U256::new(0);
        let mut board_state = 0u32;

        let mut current_board_image_pos = 0;
        let mut white_king_pos: u8 = Default::default();
        let mut black_king_pos: u8 = Default::default();
        for fen_board_char in fen_board.chars() {
            let possible_board_piece = match fen_board_char {
                'p' => Some(BLACK | PAWN),
                'n' => Some(BLACK | KNIGHT),
                'b' => Some(BLACK | BISHOP),
                'r' => Some(BLACK | ROOK),
                'q' => Some(BLACK | QUEEN),
                'k' => {
                    black_king_pos = current_board_image_pos;
                    Some(BLACK | KING)
                },
                'P' => Some(WHITE | KING),
                'N' => Some(WHITE | KNIGHT),
                'B' => Some(WHITE | BISHOP),
                'R' => Some(WHITE | ROOK),
                'Q' => Some(WHITE | QUEEN),
                'K' => {
                    white_king_pos = current_board_image_pos;
                    Some(WHITE | KING)
                },
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
                board_image |= U256::from(board_piece) << (current_board_image_pos * 4);
                current_board_image_pos += 1;
            }
        }

        let active_color = fen_parts.next().unwrap();
        let active_color_bit_flag_mask = if active_color == "w" { 1u32 << 31u32 } else { 0u32 };
        board_state |= active_color_bit_flag_mask;
        
        let castle_availibility = fen_parts.next().unwrap();
        for castle_flag in castle_availibility.chars() {
            let castle_flag_mask = match castle_flag {
                'K' => 1u32 << 30,
                'Q' => 1u32 << 29,
                'k' => 1u32 << 28,
                'q' => 1u32 << 27,
                _ => 0
            };
            board_state |= castle_flag_mask;
        }
        
        let en_passant_target = fen_parts.next().unwrap();
        if en_passant_target != "-" {
            let en_passant_target_square = BoardSquare::from(en_passant_target);
            board_state |= 1u32 << 26;
            board_state |= (en_passant_target_square.pos() as u32) << 20;
        }

        board_state |= (white_king_pos as u32) << 14;
        board_state |= (black_king_pos as u32) << 8;        

        return Board(board_image, board_state);
    }
    #[inline]
    pub fn active_color(&self) -> Color {
        let mask = 1u32 << 31u32;
        return ((self.1 | mask) >> 31u32) as u8;
    }
    #[inline]
    pub fn get_piece_at(&self, square: BoardSquare) -> Option<Piece> {
        let mask_distance_away = square.pos() * 4;
        let mask = U256::new(0b1111) << mask_distance_away;
        let square_contents = ((self.0 & mask) >> mask_distance_away).as_u8();
        if !is_piece(square_contents) {
            return None;
        }
        return Some(square_contents);
    }
    pub fn get_valid_moves(&self) -> Vec<BoardMove> {
        let mut valid_moves: Vec<BoardMove> = Vec::new();
        
        let square_of_king = 1;
        
        for origin_square_pos in 0..64 {
            let origin_square = BoardSquare(origin_square_pos);
            let possible_origin_piece = self.get_piece_at(origin_square);
            if let None = possible_origin_piece { continue; }
            let origin_piece = possible_origin_piece.unwrap();
            let origin_color = get_piece_color(origin_piece);
            let origin_piece_type = get_piece_type(origin_piece);
            match origin_piece_type {
                PAWN => {
                    let dir = if origin_color == WHITE { -1u8 } else { 1u8 };
                    
                },
            }
        }
        todo!();
    }
}
//*
fn main() {
    let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{:?}", b);
}
//*/
