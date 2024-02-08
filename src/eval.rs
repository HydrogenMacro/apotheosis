use crate::board::*;
pub fn eval_board_basic(board: Board) -> i32 {
    let BoardInfo {
        board_pieces,
        valid_moves,
        square_control
    } = board.get_board_info();
    
    let victor_side = (i32::from(board_pieces.white_king.is_some()) - i32::from(board_pieces.black_king.is_some()));
    // victor_side is 0 when both kings (or none) exist
    if victor_side {
        return victor_side * i32::MAX;
    }
    let material_evaluation: i32 = 
        (board_pieces.white_pawns.len() as i32 - board_pieces.black_pawns.len() as i32) * 1
        + (board_pieces.white_knights.len() as i32 - board_pieces.black_knights.len() as i32) * 3
        + (board_pieces.white_bishops.len() as i32 - board_pieces.black_bishops.len() as i32) * 3
        + (board_pieces.white_rooks.len() as i32 - board_pieces.black_rooks.len() as i32) * 5
        + (board_pieces.white_queens.len() as i32 - board_pieces.black_queens.len() as i32) * 9;
    
    // expressed through a number, higher means more endgame
    let game_phase = [
        (8 - board_pieces.white_pawns.len()) * 1
        + (3 - board_pieces.white_knight.len()) * 5
        + (3 - board_pieces.white_bishops.len()) * 5
        + (3 - board_pieces.white_rooks.len()) * 7
        + (1 - board_pieces.white_queens.len()) * 15,
        (8 - board_pieces.black_pawns.len()) * 1
        + (3 - board_pieces.black_knight.len()) * 5
        + (3 - board_pieces.black_bishops.len()) * 5
        + (3 - board_pieces.black_rooks.len()) * 7
        + (1 - board_pieces.black_queens.len()) * 15,
    ];
    todo!();
}