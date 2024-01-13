use crate::board::{
    Board
};
pub fn eval_board_basic(board: Board) -> i32 {
    let valid_moves = board.get_valid_moves();
    /*
    let board_info = board.get_board_info();
    let material_evaluation = 
        (board_info.white_pawns.len() - board_info.black_pawns.len()) * 1
        + (board_info.white_knights.len() - board_info.black_knights.len()) * 3
        + (board_info.white_bishops.len() - board_info.black_bishops.len()) * 3
        + (board_info.white_rooks.len() - board_info.black_rooks.len()) * 5
        + (board_info.white_queens.len() - board_info.black_queens.len()) * 9;
    let pawn_positional_evaluation = 
    */
    todo!();
}
