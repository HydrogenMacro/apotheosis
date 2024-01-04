use std::{
    collections::HashSet,
};
use ethnum::*;

pub type square = u8;
pub mod BoardSquare {
    use crate::*;
    #[inline]
    pub const fn from(board_sqaure_notation: &str) -> square {
        if board_square_notation.len() != 2 {
            panic!("BoardSquare::from takes a 2 lengthed string, like \"d3\"");
        }
        if let [row, col] = board_square_notation.as_bytes() {
            let row_value: u32 = match row {
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
            let col_value: u32 = match col {
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
            return row_value + col_value;
        }
        panic!("BoardSquare::from() failed");
    }
    #[inline]
    pub const fn get_x_pos_of(square_to_check: square) -> u8 {
        
    }
    #[inline]
    pub const fn get_y_pos_of(square_to_check: square) -> u8 {
        
    }
    #[inline]
    pub const fn get_square_in_direction(origin_square: square, dx: isize, dy: isize) -> Option<square> {
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
        
        return Some(resulting_square as square);
    }
    #[inline]
    pub const fn get_square_above(origin_square: square) -> Option<square> {
        let resulting_square = origin_square - 8;
        if resulting_square < 0 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn get_square_below(origin_square: square) -> Option<square> {
        let resulting_square = origin_square + 8;
        if resulting_square >= 64 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn get_square_left_of(origin_square: square) -> Option<square> {
        let resulting_square = origin_square - 1;
        if resulting_square % 8 == 7 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn get_square_right_of(origin_square: square) -> Option<square> {
        let resulting_square = origin_square + 1;
        if resulting_square % 8 == 0 {
            return None
        }
        return Some(resulting_square);
    }
    #[inline]
    pub const fn are_colinear(square_a: square, square_b: square) -> bool {
        
    }
}

pub type color = u8;
pub mod BoardColor {
    use crate::*;
    pub const BLACK: color = 0b0000;
    pub const WHITE: color = 0b0001;
}

pub type piecetype = u8;
pub mod BoardPieceType {
    use crate::*;
    pub const PAWN: piecetype = 0b0010;
    pub const KNIGHT: piecetype = 0b0100;
    pub const BISHOP: piecetype = 0b0110;
    pub const ROOK: piecetype = 0b1000;
    pub const QUEEN: piecetype = 0b1010;
    pub const KING: piecetype = 0b1100;
}

pub type piece = u8;
pub mod BoardPiece {
    use crate::*;
    pub const BLANK: piece = 0b0000;
    pub const BLACK_PAWN: piece = 0b0010;
    pub const BLACK_KNIGHT: piece = 0b0100;
    pub const BLACK_BISHOP: piece = 0b0110;
    pub const BLACK_ROOK: piece = 0b1000;
    pub const BLACK_QUEEN: piece = 0b1010;
    pub const BLACK_KING: piece = 0b1100;
    pub const WHITE_PAWN: piece = 0b0011;
    pub const WHITE_KNIGHT: piece = 0b0101;
    pub const WHITE_BISHOP: piece = 0b0111;
    pub const WHITE_ROOK: piece = 0b1001;
    pub const WHITE_QUEEN: piece = 0b1011;
    pub const WHITE_KING: piece = 0b1101;
    
    #[inline]
    pub const fn color(board_piece: piece) -> color {
        return board_piece & 0b1000u8;
    }
    #[inline]
    pub const fn piece_type(board_piece: piece) -> piecetype {
        return board_piece & 0b0111u8;
    }
    #[inline]
    pub const fn is_piece(board_piece: piece) -> bool {
        return board_piece != BLANK;
    }
}

#[derive(PartialEq, Debug)]
pub struct BoardMove(u16);

impl BoardMove {
    pub const CASTLE_BQ: BoardMove = BoardMove(0b1000_0000_0000_0000u16);
    pub const CASTLE_BK: BoardMove = BoardMove(0b1010_0000_0000_0000u16);
    pub const CASTLE_WQ: BoardMove = BoardMove(0b1100_0000_0000_0000u16);
    pub const CASTLE_WK: BoardMove = BoardMove(0b1110_0000_0000_0000u16);
    
    #[inline]
    pub fn from_board_squares(origin_square: square, dest_square: square) -> BoardMove {
        return BoardMove(
                ((origin_square as u16) << 9) 
                | ((dest_square as u16) << 3)
        );
    }
    #[inline]
    pub fn from_board_squares_as_en_passant(origin_square: u8, dest_square: u8) -> BoardMove {
        return BoardMove(
            0b0000_0000_0000_0100u16
                | ((origin_square as u16) << 9)
                | ((dest_square as u16) << 3)
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
        let mut board_image = U256::new();
        let mut board_state = 0u32;

        let mut current_board_image_pos = 0;
        let mut white_king_pos: square;
        let mut black_king_pos: square;
        for fen_board_char in fen_board.chars() {
            let possible_board_piece = match fen_board_char {
                'p' => Some(BoardPiece::BLACK_PAWN),
                'n' => Some(BoardPiece::BLACK_KNIGHT),
                'b' => Some(BoardPiece::BLACK_BISHOP),
                'r' => Some(BoardPiece::BLACK_ROOK),
                'q' => Some(BoardPiece::BLACK_QUEEN),
                'k' => {
                    black_king_pos = current_board_image_pos;
                    Some(BoardPiece::BLACK_KING)
                },
                'P' => Some(BoardPiece::WHITE_PAWN),
                'N' => Some(BoardPiece::WHITE_KNIGHT),
                'B' => Some(BoardPiece::WHITE_BISHOP),
                'R' => Some(BoardPiece::WHITE_ROOK),
                'Q' => Some(BoardPiece::WHITE_QUEEN),
                'K' => {
                    white_king_pos = current_board_image_pos;
                    Some(BoardPiece::WHITE_KING)
                },
                _ => None
            };
            if let Some(board_piece) = possible_board_piece {
                board_image |= ((board_piece as u256) << current_board_image_pos * 4);
                current_board_image_pos += 1;
            } else {
                match possible_board_piece {
                    '1' => current_board_image_pos += 1,
                    '2' => current_board_image_pos += 2,
                    '3' => current_board_image_pos += 3,
                    '4' => current_board_image_pos += 4,
                    '5' => current_board_image_pos += 5,
                    '6' => current_board_image_pos += 6,
                    '7' => current_board_image_pos += 7,
                    '8' => current_board_image_pos += 8,
                    _ | '/' => {}
                }
            }
        }

        let active_color = fen_parts.next().unwrap();
        let active_color_bit_flag_mask = if active_color == "w" { 1u32 << 31u32 } else { 0u32 };
        board_state |= active_color_bit_flag_mask;
        
        let castle_availibility = fen_parts.next().unwrap();
        for castle_flag in castle_availibility.char() {
            let castle_flag_mask = match castle_flag {
                'K' => 1u32 << 30,
                'Q' => 1u32 << 29,
                'k' => 1u32 << 28,
                'q' => 1u32 << 27,
            };
            board_state |= castle_flag_mask;
        }
        
        let en_passant_target = fen_parts.next().unwrap();
        if en_passant_target != "-" {
            let en_passant_target_square = BoardSquare::from(en_passant_target_square);
            board_state |= 1u32 << 26;
            board_state |= en_passant_target_square << 20;
        }

        board_state |= white_king_pos << 14;
        board_state |= black_king_pos << 8;        

        return Board(board_image, board_state);
    }
    #[inline]
    pub fn active_color(&self) -> color {
        let mask = 1u32 << 31u32;
        return ((self.0 | mask) >> 31u32) as color;
    }
    #[inline]
    pub fn get_piece_at(&self, square: square) -> Option<piece> {
        let mask_distance_away = square * 4;
        let mask = U256::new(0b1111) << mask_distance_away;
        let square_contents = ((self.0 & mask) >> mask_distance_away) as piece;
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
                        for base_delta in 1..8 {
                            let reachable_squares = [
                                BoardSquare::get_square_in_direction(1 * base_delta, 1 * base_delta),
                                BoardSquare::get_square_in_direction(1 * base_delta, -1 * base_delta),
                                BoardSquare::get_square_in_direction(-1 * base_delta, 1 * base_delta),
                                BoardSquare::get_square_in_direction(-1 * base_delta, -1 * base_delta),
                            ];
                            for dir in 0..4 {
                                let reachable_square = reachable_squares[dir];
                                let reachable_piece = self.get_piece_at(reachable_square);
                                if reachable_piece.is_piece() {
                                    if origin_board_piece.color() != reachable_piece.color() {
                                        
                                    } else {
                                        break;
                                    }
                                } else {
                                    
                                }
                            }
                        }
                    },
                    BoardPieceType::KNIGHT => {
                        
                    },
                    BoardPieceType::ROOK => {
                        
                    },
                    BoardPieceType::QUEEN => {
                        
                    },
                    BoardPieceType::KING => {
                        
                    },
                    _ => panic!("{:b} is not a valid piece", origin_board_piece)
                }
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
