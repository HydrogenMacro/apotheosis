use crate::board::{
    Board
};
pub fn eval_board_basic(board: Board) -> i32 {
    let valid_moves = board.get_valid_moves();
    /*
    let board_info = board.get_board_info();
    let material_evaluation: i32 = 
        (board_info.white_pawns.len() as i32 - board_info.black_pawns.len() as i32) * 1
        + (board_info.white_knights.len() as i32 - board_info.black_knights.len() as i32) * 3
        + (board_info.white_bishops.len() as i32 - board_info.black_bishops.len() as i32) * 3
        + (board_info.white_rooks.len() as i32 - board_info.black_rooks.len() as i32) * 5
        + (board_info.white_queens.len() as i32 - board_info.black_queens.len() as i32) * 9
        + (i32::from(board_info.white_king.is_some()) - i32::from(board_info.black_king.is_some())) * 200;
    */
    todo!();
}