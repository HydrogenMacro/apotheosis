use apotheosis::board::{
    Board,
    *
};
use std::{
    collections::HashSet, 
    hash::Hash,
    iter::FromIterator,
    clone::Clone,
    fmt::Debug
};

#[test]
fn movement_tests() {
    let pawn_test_board = Board::from_fen(
        "8/8/8/8/8/2n5/1P6/8 w - - 0 1"
    );
    println!("pawn board:\n{}", pawn_test_board);
    let pawn_test_board_valid_moves = vec![
        boardmove("b2", "b3"),
        boardmove("b2", "c3"),
        boardmove("b2", "b4")
    ];
    let pawn_test_board_found_moves = pawn_test_board.get_valid_moves();
    for m in pawn_test_board_found_moves.iter() {
        println!("{}", m);
    }
    assert_eq!(
        pawn_test_board_found_moves, 
        pawn_test_board_valid_moves
    );
}

fn boardmove(s1: &str, s2: &str) -> BoardMove {
    return BoardMove::new(&BoardSquare::from(s1), &BoardSquare::from(s2));
}
