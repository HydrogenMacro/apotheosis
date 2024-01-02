use std::{
    collections::HashSet,
};
use ethnum::*;

pub type BoardSquare = u8;
pub mod BoardSquare {
    #[inline]
    pub const fn get_square_in_direction(origin_square: BoardSquare, dx: isize, dy: isize) -> Option<BoardSquare> {
        let origin_square_x = origin_square % 8;
        let origin_square_y = (origin_square - origin_square_x) / 8;
        
        let resulting_square = origin_square as isize + dx + (-8 * dy);
        let resulting_square_x = origin_square_x as isize + dx;
        let resulting_square_y = origin_square_y as isize - dy;
        
        if resulting_square_x < 0 || resulting_square_x >= 8 {
            return None;
        }
        if resulting_square_y < 0 || resulting_square_y >= 8 {
            return None;
        }
        
        return Some(resulting_square as BoardSquare);
    }
    #[inline]
    pub const fn get_square_above(origin_square: BoardSquare) -> Option<BoardSquare> {
        let resulting_square = origin_square - 8;
        if resulting_square < 0 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn get_square_below(origin_square: BoardSquare) -> Option<BoardSquare> {
        let resulting_square = origin_square + 8;
        if resulting_square >= 64 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn get_square_left_of(origin_square: BoardSquare) -> Option<BoardSquare> {
        let resulting_square = origin_square - 1;
        if resulting_square % 8 == 7 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn get_square_right_of(origin_square: BoardSquare) -> Option<BoardSquare> {
        let resulting_square = origin_square + 1;
        if resulting_square % 8 == 0 {
            return None
        }
        return Some(resulting_square);
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
    pub const fn color(&self) -> BoardColor {
        return self & 0b1000u8;
    }
    #[inline]
    pub const fn piece_type(&self) -> BoardPieceType {
        return self & 0b0111u8;
    }
    #[inline]
    pub const fn is_piece(&self) -> bool {
        return self.piece_type != BLANK;
    }
}

pub struct BoardMove(u16);

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

pub struct Board(u256, u32);
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
        let mut valid_moves: Vec<BoardMove> = Vec::new();
        
        let square_of_king = 1;
        
        for origin_board_square in 0..64 {
            if let Some(origin_board_piece) = self.get_piece_at(origin_board_piece) {
                if !origin_board_piece.is_piece() {
                    continue;
                }
                match origin_board_piece.piece_type() {
                    BoardPieceType::PAWN => {
                        let is_white = origin_board_piece.color() == BoardColor::White;
                        let possible_short_move_resulting_square = if is_white {
                            get_square_above(origin_board_square)
                        } else {
                            get_square_below(origin_board_square)
                        };
                        if let Some(short_move_resulting_square) = possible_short_move_resulting_square {
                            if !self.get_piece_at(short_move_resulting_square).is_piece() {
                                valid_moves.push(BoardMove::from_board_squares(origin_board_square, short_move_resulting_square));
                            }
                        }
                        
                        let possible_extended_move_resulting_square = if is_white {
                            get_square_in_direction(origin_board_square, 0, 2)
                        } else {
                            get_square_in_direction(origin_board_square, 0, -2)
                        };
                        if let Some(extended_move_resulting_square) = possible_extended_move_resulting_square {
                            if !self.get_piece_at(extended_move_resulting_square).is_piece() {
                                valid_moves.push(BoardMove::from_board_squares(origin_board_square, extended_move_resulting_square));
                            }
                        }
                        
                        let possible_left_capture_resulting_square = if is_white {
                            get_square_in_direction(origin_board_square, -1, 1)
                        } else {
                            get_square_in_direction(origin_board_square, 1, -1)
                        };
                        if let Some(left_capture_resulting_square) = possible_left_capture_resulting_square {
                            let piece_to_capture = self.get_piece_at(left_capture_resulting_square);
                            if piece_to_capture.is_piece() && piece_to_capture.color() != origin_board_piece.color() {
                                valid_moves.push(BoardMove::from_board_squares(origin_board_square, left_capture_resulting_square));
                            }
                        }
                        
                        let possible_right_capture_resulting_square = if is_white {
                            get_square_in_direction(origin_board_square, 1, 1)
                        } else {
                            get_square_in_direction(origin_board_square, -1, -1)
                        };
                        if let Some(right_capture_resulting_square) = possible_right_capture_resulting_square {
                            let piece_to_capture = self.get_piece_at(right_capture_resulting_square);
                            if piece_to_capture.is_piece() && piece_to_capture.color() != origin_board_piece.color() {
                                valid_moves.push(BoardMove::from_board_squares(origin_board_square, right_capture_resulting_square));
                            }
                        }
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