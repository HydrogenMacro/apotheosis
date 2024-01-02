use std::{
    collections::HashSet,
};
use ethnum::prelude;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
#[derive(Debug, Copy, Clone, Default)]
pub enum BoardSquare {
    Occupied(Color, Piece),
    #[default]
    Blank
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    White
}
use Color::*;
use Piece::*;
#[derive(Debug)]
pub struct Board {
    data: [BoardSquare; 64],
    turn: Color,
    castle_availibility: [[bool; 2]; 2],
    halfmove_clock: u8,
    en_passant_target_square: Option<u8>,
    move_num: u64,
}
impl Default for Board {
    fn default() -> Board {
        return Board {
            data: [BoardSquare::Blank; 64],
            turn: White,
            castle_availibility: [[false, false], [false, false]],
            halfmove_clock: 0,
            en_passant_target_square: None,
            move_num: 1
        }
    }
}
impl Board {
    pub fn from_fen(fen: &str) -> Board {
        let mut state: u8 = 0;
        let mut board = Board::default();
        let mut en_passant_target_square: u8 = 0;
        let mut halfmove_clock: u8 = 0;
        let mut move_num: u64 = 0;
        let mut current_board_index = 0;
        for c in fen.chars() {
            if c == ' ' {
                state += 1;
                continue;
            }
            match state {
                0 => { // board state
                    //println!("{} ? {}", c, current_board_index);
                    match c {
                        'p' => board.data[current_board_index] = BoardSquare::Occupied(Black, Pawn),
                        'b' => board.data[current_board_index] = BoardSquare::Occupied(Black, Bishop),
                        'n' => board.data[current_board_index] = BoardSquare::Occupied(Black, Knight),
                        'r' => board.data[current_board_index] = BoardSquare::Occupied(Black, Rook),
                        'q' => board.data[current_board_index] = BoardSquare::Occupied(Black, Queen),
                        'k' => board.data[current_board_index] = BoardSquare::Occupied(Black, King),
                        'P' => board.data[current_board_index] = BoardSquare::Occupied(White, Pawn),
                        'B' => board.data[current_board_index] = BoardSquare::Occupied(White, Bishop),
                        'N' => board.data[current_board_index] = BoardSquare::Occupied(White, Knight),
                        'R' => board.data[current_board_index] = BoardSquare::Occupied(White, Rook),
                        'Q' => board.data[current_board_index] = BoardSquare::Occupied(White, Queen),
                        'K' => board.data[current_board_index] = BoardSquare::Occupied(White, King),
                        _ if matches!(c.to_digit(10), Some(blank_amount)) => current_board_index += (c.to_digit(10).expect("no") as usize) - 1,
                        '/' | _ => continue,
                    }
                    current_board_index += 1;
                },
                1 => { //player to move
                    board.turn = if c == 'w' { White } else { Black };
                },
                2 => { //castling availibility
                    match c {
                        'K' => board.castle_availibility[0][0] = true,
                        'Q' => board.castle_availibility[0][1] = true,
                        'k' => board.castle_availibility[1][0] = true,
                        'q' => board.castle_availibility[1][1] = true,
                        _ => {}
                    }
                },
                3 => { // en passant target square
                    let v: u8 = match c {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ if matches!(c.to_digit(10), Some(blank_amount)) => (8u8 - (c.to_digit(10).expect("no") as u8)) * 8u8,
                        '-' => 69,
                        _ => unreachable!()
                    };
                    if v == 69 {
                        board.en_passant_target_square = None;
                    } else {
                        en_passant_target_square += v;
                        board.en_passant_target_square = Some(en_passant_target_square);
                    }
                },
                4 => { // half move clock
                    halfmove_clock = halfmove_clock * 10 + (c.to_digit(10).unwrap() as u8);
                    board.halfmove_clock = halfmove_clock;
                },
                5 => { // move num
                    move_num = move_num * 10 + (c.to_digit(10).unwrap() as u64);
                    board.move_num = move_num;
                },
                _ => {}
            }
        }
        return board;
    }
    pub fn get_valid_moves(&self) -> Vec<(i8, i8)> {
        // sorry in advance for the code here
        let mut valid_moves: Vec<(i8, i8)> = Vec::with_capacity(36);
        for target_piece_square_index in 0..64 {
            let target_piece = self.data[target_piece_square_index as usize];
            if let BoardSquare::Occupied(target_piece_color, target_piece_type) = target_piece {
                match target_piece_type {
                    Pawn => {
                        let direction = if target_piece_color == White { -1i8 } else { 1i8 };
                        let reachable_square_index = target_piece_square_index + (direction * 8);
                        if reachable_square_index >= 0 && reachable_square_index < 64 {
                            if let BoardSquare::Blank = self.data[reachable_square_index as usize] {
                                valid_moves.push((target_piece_square_index, reachable_square_index));
                            }
                        }
                        let is_on_starting_square = match target_piece_color {
                            White => target_piece_square_index >= 48 && target_piece_square_index < 56,
                            Black => target_piece_square_index >= 8  && target_piece_square_index < 16
                        };
                        if is_on_starting_square {
                            let extended_reachable_square_index = target_piece_square_index + (direction * 16);
                            if extended_reachable_square_index >= 0 && extended_reachable_square_index < 64 {
                                if let BoardSquare::Blank = self.data[extended_reachable_square_index as usize] {
                                    valid_moves.push((target_piece_square_index, extended_reachable_square_index));
                                }
                            }
                        }
                        let base_capture_deltas = [7, 9];
                        for base_capture_delta in capture_deltas.iter() {
                            let capture_delta = base_capture_delta * direction;
                            let capturable_square_index = target_piece_square_index + capture_delta;
                            if capturable_square_index < 0 || capturable_square_index >= 64 {
                                break;
                            }
                            if ((target_piece_square_index % 8) - (capturable_square_index % 8)).abs() == 7 {
                                break;
                            }
                            if let BoardSquare::Occupied(capturable_piece_color, capturable_piece_type) = self.data[capturable_square_index as usize] {
                                if capturable_piece_color != target_piece_color {
                                    valid_moves.push((target_piece_square_index, capturable_square_index));
                                }
                            } else {
                                if let Some(en_passant_target_square_index) = self.en_passant_target_square {
                                    if en_passant_target_square_index == capturable_square_index {
                                        if let BoardSquare::Occupied(adjacent_piece_color, adjacent_piece_type) = self.data[target_piece_square_index + direction] {
                                            if adjacent_piece_color != target_piece_color {
                                                if let Pawn = adjacent_piece_type {
                                                    valid_moves.push((target_piece_square_index, en_passant_target_square_index));
                                                    // TODO: this captures
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    Knight | King => {
                        let deltas = match target_piece_type {
                            Knight => [10, 17, 15, 6, -10, -17, -15, -6],
                            King => [-9, -8, -7, -1, 1, 7, 8, 9],
                            _ => unreachable!()
                        };
                        for delta in deltas.iter() {
                            let reachable_square_index = target_piece_square_index + delta;
                            if reachable_square_index < 0 || reachable_square_index >= 64 {
                                break;
                            }
                            if ((target_piece_square_index % 8) - (reachable_square_index % 8)).abs() >= 5 {
                                // please be correct condition
                                break;
                            }
                            let reachable_square = self.data[reachable_square_index as usize];
                            if let BoardSquare::Occupied(piece_in_range_color, piece_in_range_type) = reachable_square {
                                if piece_in_range_color == target_piece_color {
                                    break;
                                }
                            }
                            valid_moves.push((target_piece_square_index, reachable_square_index));
                        }
                    },
                    Bishop | Rook | Queen => {
                        let directions = match target_piece_type {
                            Bishop => &[-7, -9, 7, 9][..],
                            Rook => &[-1, -8, 1, 8][..],
                            Queen => &[-1, -7, -8, -9, 1, 7, 8, 9][..],
                            _ => unreachable!()
                        };
                        for base_delta in 1..=8 {
                            for dir in directions.iter() {
                                let delta = base_delta * dir;
                                let reachable_square_index = target_piece_square_index + delta;
                                if reachable_square_index < 0 || reachable_square_index >= 64 {
                                    break;
                                }
                                if ((target_piece_square_index % 8) - (reachable_square_index % 8)).abs() == 7 {
                                    // yes
                                    break;
                                }
                                let reachable_square = self.data[reachable_square_index as usize];
                                if let BoardSquare::Occupied(piece_in_range_color, piece_in_range_type) = reachable_square {
                                    if piece_in_range_color != target_piece_color { // if piece in range is capturable
                                        valid_moves.push((target_piece_square_index, reachable_square_index));
                                    }
                                    break;
                                } else {
                                    println!("{} to {} is valid", target_piece_square_index, reachable_square_index);
                                    valid_moves.push((target_piece_square_index, reachable_square_index));
                                }
                            }
                        }
                    }
                }
            }
        }
        // castling
        return valid_moves;
    }
    pub fn generate_board_from_piece_move(&self) {
        todo!();
    }
    pub fn print(&self) {
        let mut board_render = String::new();
        for i in 0..64 {
            let board_space = self.data[i];
            if let BoardSquare::Occupied(color, piece) = board_space {
                let piece_as_char = match piece {
                    Pawn => if color == White { 'P' } else { 'p' },
                    Knight => if color == White { 'N' } else { 'n' },
                    Bishop => if color == White { 'B' } else { 'b' },
                    Rook => if color == White { 'R' } else { 'r' },
                    Queen => if color == White { 'Q' } else { 'q' },
                    King => if color == White { 'K' } else { 'k' }
                };
                board_render.push(piece_as_char);
            }
            if i % 8 == 7 { board_render.push('\n') }
        }
        println!("{}", board_render);
    }
}