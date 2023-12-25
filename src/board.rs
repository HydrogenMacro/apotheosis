#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
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
    move_num: u64
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
                        'b' => 8,
                        'c' => 16,
                        'd' => 24,
                        'e' => 32,
                        'f' => 40,
                        'g' => 48,
                        'h' => 56,
                        _ if matches!(c.to_digit(10), Some(blank_amount)) => c.to_digit(10).expect("no") as u8,
                        _ => 69
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
    pub fn get_valid_moves(&self) -> &[u8] {
        for i in 1..=2 {
            println!("{:?}", Black);
        }
        todo!();
    }
    pub fn check_move(&self) -> bool {
        
        todo!();
    }
    pub fn get_piece_at(&self) -> Piece {
        todo!();
    }
    pub fn print(&self) {
        todo!();
    }
}