use std::{
    collections::HashSet,
};
use ethnum::*;

pub struct BoardMove(u16);
pub struct Board(u256, u32);

pub type BoardSquare = u8;
pub mod BoardSquare {
    #[inline]
    pub fn get_square_in_direction(origin_square: BoardSquare, dx: isize, dy: isize) -> Option<BoardSquare> {
        let resulting_square = origin_square + dx + (8 * dy);
        if resulting_square < 0 || resulting_square >= 64 {
            return None;
        }
        return Some(resulting_square);
    }
    #[inline]
    pub fn get_square_above() {
        
    }
}

pub type BoardColor = u8;
pub mod BoardColor {
    pub const BLACK: BoardColor = 0b0000;
    pub const WHITE: BoardColor = 0b1000;
}

pub type BoardPieceType = u8;
pub mod BoardPieceType {
    pub const PAWN: BoardPieceType = 0b0001;
    pub const KNIGHT: BoardPieceType = 0b0010;
    pub const BISHOP: BoardPieceType = 0b0011;
    pub const ROOK: BoardPieceType = 0b0100;
    pub const QUEEN: BoardPieceType = 0b0101;
    pub const KING: BoardPieceType = 0b0111;
}

pub type BoardPiece = u8;
pub mod BoardPiece {
    pub const BLANK: BoardPiece = 0b0000;
    pub const BLACK_PAWN: BoardPiece = 0b0001;
    pub const BLACK_KNIGHT: BoardPiece = 0b0010;
    pub const BLACK_BISHOP: BoardPiece = 0b0011;
    pub const BLACK_ROOK: BoardPiece = 0b0100;
    pub const BLACK_QUEEN: BoardPiece = 0b0101;
    pub const BLACK_KING: BoardPiece = 0b0111;
    pub const WHITE_PAWN: BoardPiece = 0b1001;
    pub const WHITE_KNIGHT: BoardPiece = 0b1010;
    pub const WHITE_BISHOP: BoardPiece = 0b1011;
    pub const WHITE_ROOK: BoardPiece = 0b1100;
    pub const WHITE_QUEEN: BoardPiece = 0b1101;
    pub const WHITE_KING: BoardPiece = 0b1111;
    
    #[inline]
    pub fn color(&self) -> BoardColor {
        return self & 0b1000u8;
    }
    #[inline]
    pub fn piece_type(&self) -> BoardPieceType {
        return self & 0b0111u8;
    }
}

impl BoardMove {
    pub const CASTLE_BQ: BoardMove = BoardMove(0b1000_0000_0000_0000u16);
    pub const CASTLE_BK: BoardMove = BoardMove(0b1010_0000_0000_0000u16);
    pub const CASTLE_WQ: BoardMove = BoardMove(0b1100_0000_0000_0000u16);
    pub const CASTLE_WK: BoardMove = BoardMove(0b1110_0000_0000_0000u16);
    
    #[inline]
    pub fn from_board_squares(origin_square: BoardSquare, dest_square: BoardSquare) -> BoardMove {
        return BoardMove(
                ((origin_square as u16) << 9) 
                | ((dest_square as u16) << 3)
        );
    }
    #[inline]
    pub fn from_board_squares_as_en_passant(origin_square: u8, dest_square: u8) -> BoardSquare {
        return BoardMove(
            0b0000_0000_0000_0100u16
                | ((origin_square as u16) << 9)
                | ((dest_square as u16) << 3)
        );
        // maybe just provide a mask
    }
}
impl Board {
    pub fn from_fen(fen: &str) -> Board {
        todo!();
    }
    #[inline]
    pub fn get_piece_at(&self, square: BoardSquare) -> Option<BoardPiece> {
        let mask_distance_away = square * 4;
        let mask = U256::new(0b1111) << mask_distance_away;
        let square_contents = ((self.0 & mask) >> mask_distance_away) as BoardPiece;
        if square_contents == BoardPiece::BLANK {
            return None;
        }
        return Some(square_contents);
    }
    pub fn get_valid_moves(&self) -> Vec<BoardMove> {
        let valid_moves: Vec<BoardMove> = Vec::new();
        for origin_board_square in 0..64 {
            if let Some(origin_board_piece) = self.get_piece_at(origin_board_piece) {
                match origin_board_piece.piece_type() {
                    BoardPieceType::PAWN => {
                        
                    },
                    BoardPieceType::BISHOP => {
                        
                    },
                    BoardPieceType::KNIGHT => {
                        
                    },
                    BoardPieceType::ROOK => {
                        
                    },
                    BoardPieceType::QUEEN => {
                        
                    },
                    BoardPieceType::KING => {
                        
                    },
                    _ => panic!("{:b}" is not a valid piece, origin_board_piece)
                }
            }
        }
        todo!();
    }
}