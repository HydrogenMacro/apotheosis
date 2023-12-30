use apotheosis::board::Board;
use std::{
    collections::HashSet, 
    hash::Hash,
    iter::FromIterator
};

#[test]
fn pawn_movement_test() {
    let a7_pawn_board = Board::from_fen("8/p7/8/8/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(a7_pawn_board.get_valid_moves(), vec![(sq("a7"), sq("a6")), (sq("a7"), sq("a5"))]);
    let a6_pawn_board = Board::from_fen("8/8/p6/8/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(a7_pawn_board.get_valid_moves(), vec![(sq("a7"), sq("a6"))]);
}

#[test]
fn knight_movement_test() {
    let d5_knight_board = Board::from_fen("8/8/8/3n4/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(d5_knight_board.get_valid_moves(), vec![
        (sq("d5"), sq("c7")),
        (sq("d5"), sq("e7")),
        (sq("d5"), sq("b6")),
        (sq("d5"), sq("f6")),
        (sq("d5"), sq("b4")),
        (sq("d5"), sq("f4")),
        (sq("d5"), sq("c3")),
        (sq("d5"), sq("e3")),
    ]);
}
#[test]
fn bishop_movement_test() {
    let d5_bishop_board = Board::from_fen("8/8/8/3b4/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(d5_bishop_board.get_valid_moves(), vec![
        (sq("d5"), sq("a8")),
        (sq("d5"), sq("b7")),
        (sq("d5"), sq("c6")),
        (sq("d5"), sq("e4")),
        (sq("d5"), sq("f3")),
        (sq("d5"), sq("g2")),
        (sq("d5"), sq("a2")),
        (sq("d5"), sq("b3")),
        (sq("d5"), sq("c4")),
        (sq("d5"), sq("e6")),
        (sq("d5"), sq("f7")),
        (sq("d5"), sq("g8"))
    ]);
}
#[test]
fn rook_movement_test() {
    let d5_rook_board = Board::from_fen("8/8/8/3r4/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(d5_rook_board.get_valid_moves(), vec![
        (sq("d5"), sq("a5")),
        (sq("d5"), sq("b5")),
        (sq("d5"), sq("c5")),
        (sq("d5"), sq("e5")),
        (sq("d5"), sq("f5")),
        (sq("d5"), sq("g5")),
        (sq("d5"), sq("h5")),
        (sq("d5"), sq("d1")),
        (sq("d5"), sq("d2")),
        (sq("d5"), sq("d3")),
        (sq("d5"), sq("d4")),
        (sq("d5"), sq("d6")),
        (sq("d5"), sq("d7")),
        (sq("d5"), sq("d8"))
    ]);
}
#[test]
fn queen_movement_test() {
    let d5_queen_board = Board::from_fen("8/8/8/3q4/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(d5_queen_board.get_valid_moves(), vec![
        (sq("d5"), sq("a5")),
        (sq("d5"), sq("b5")),
        (sq("d5"), sq("c5")),
        (sq("d5"), sq("e5")),
        (sq("d5"), sq("f5")),
        (sq("d5"), sq("g5")),
        (sq("d5"), sq("h5")),
        (sq("d5"), sq("d1")),
        (sq("d5"), sq("d2")),
        (sq("d5"), sq("d3")),
        (sq("d5"), sq("d4")),
        (sq("d5"), sq("d6")),
        (sq("d5"), sq("d7")),
        (sq("d5"), sq("d8")),
        (sq("d5"), sq("a8")),
        (sq("d5"), sq("b7")),
        (sq("d5"), sq("c6")),
        (sq("d5"), sq("e4")),
        (sq("d5"), sq("f3")),
        (sq("d5"), sq("g2")),
        (sq("d5"), sq("a2")),
        (sq("d5"), sq("b3")),
        (sq("d5"), sq("c4")),
        (sq("d5"), sq("e6")),
        (sq("d5"), sq("f7")),
        (sq("d5"), sq("g8"))
    ]);
}
#[test]
fn king_movement_test() {
    let d5_king_board = Board::from_fen("8/8/8/3k4/8/8/8/8 w KQkq - 0 1");
    assert_vecs_are_permutations(d5_king_board.get_valid_moves(), vec![
        (sq("d5"), sq("c6")),
        (sq("d5"), sq("d6")),
        (sq("d5"), sq("e6")),
        (sq("d5"), sq("c5")),
        (sq("d5"), sq("e5")),
        (sq("d5"), sq("c4")),
        (sq("d5"), sq("d4")),
        (sq("d5"), sq("e4")),
    ]);
}
fn sq(board_square: &str) -> i8 {
    let mut board_square_chars = board_square.chars();
    if let (Some(col), Some(row)) = (board_square_chars.next(), board_square_chars.next()) {
        let col_value = match col {
            'a' => 0,
            'b' => 8,
            'c' => 16,
            'd' => 24,
            'e' => 32,
            'f' => 40,
            'g' => 48,
            'h' => 56,
            _ => unreachable!()
        };
        let row_value = row.to_digit(10).unwrap() - 1;
        return col_value as i8 + row_value as i8;
    }
    panic!("board_square should only have 2 characters");
}
fn assert_vecs_are_permutations<T: Eq + Hash>(vec1: Vec<T>, vec2: Vec<T>) {
    let set1 = HashSet::from_iter(vec1);
    let vecs_are_permutations = vec1.len() == vec2.len() && vec2.iter().all(|e| set1.contains(e));
    assert!(vecs_are_permutations);
}