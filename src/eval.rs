use crate::board::{
    Board
};
pub fn eval_board(board: Board, depth: u32) -> i32 {
    
    let board_info = board.get_board_info();
    
    let victor_side = (i32::from(board_info.white_king.is_some()) - i32::from(board_info.black_king.is_some()));
    // victor_side is 0 when both kings (or none) exist
    if victor_side {
        return victor_side * i32::MAX;
    }
    let material_evaluation: i32 = 
        (board_info.white_pawns.len() as i32 - board_info.black_pawns.len() as i32) * 1
        + (board_info.white_knights.len() as i32 - board_info.black_knights.len() as i32) * 3
        + (board_info.white_bishops.len() as i32 - board_info.black_bishops.len() as i32) * 3
        + (board_info.white_rooks.len() as i32 - board_info.black_rooks.len() as i32) * 5
        + (board_info.white_queens.len() as i32 - board_info.black_queens.len() as i32) * 9;
    
    // positional eval, etc
    
    if depth == 0 {
        return todo!();
    }
    
    // keep track of best / most volatile
    for valid_move in board_info.valid_moves {
        let resulting_board = board.create_board_from_move(valid_move);
        let resulting_board_evaluation = eval_board(resulting_board, depth - 1);
        
    }
    
    todo!();
}