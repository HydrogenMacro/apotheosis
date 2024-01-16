use apotheosis::board::{
    Board,
    *
};
use std::{
    collections::HashSet, 
    hash::Hash,
    iter::FromIterator,
    clone::Clone,
    fmt
};

#[test]
fn pawn_movement_test() {
    let test_board = Board::from_fen(
        "8/8/8/8/8/2n5/1P6/8 w - - 0 1"
    );
    let test_board_valid_moves = vec![
        boardmove("b2", "b3"),
        boardmove("b2", "c3"),
        boardmove("b2", "b4")
    ];
    let test_board_found_moves = test_board.get_valid_moves();
    assert_eq_lists(
        &test_board_found_moves, 
        &test_board_valid_moves
    );
}

#[test]
fn knight_movement_test() {
    let test_board = Board::from_fen(
        "8/8/8/p7/P7/3q4/1N6/8 w - - 0 1"
    );
    let test_board_valid_moves = vec![
        boardmove("b2", "c4"),
        boardmove("b2", "d3"),
        boardmove("b2", "d1")
    ];
    let test_board_found_moves = test_board.get_valid_moves();
    assert_eq_lists(
        &test_board_found_moves, 
        &test_board_valid_moves
    );
}

#[test]
fn bishop_movement_test() {
    let test_board = Board::from_fen(
        "8/8/5p2/5P2/8/3B4/4q3/8 w - - 0 1"
    );
    let test_board_valid_moves = vec![
        boardmove("d3", "b1"),
        boardmove("d3", "c2"),
        boardmove("d3", "e4"),
        boardmove("d3", "a6"),
        boardmove("d3", "b5"),
        boardmove("d3", "c4"),
        boardmove("d3", "e2")
    ];
    let test_board_found_moves = test_board.get_valid_moves();
    assert_eq_lists(
        &test_board_found_moves, 
        &test_board_valid_moves
    );
}

#[test]
fn rook_movement_test() {
    let test_board = Board::from_fen(
        "8/8/8/2p5/2P5/8/2R3q1/8 w - - 0 1"
    );
    let test_board_valid_moves = vec![
        boardmove("c2", "a2"),
        boardmove("c2", "b2"),
        boardmove("c2", "d2"),
        boardmove("c2", "e2"),
        boardmove("c2", "f2"),
        boardmove("c2", "g2"),
        boardmove("c2", "c1"),
        boardmove("c2", "c3")
    ];
    let test_board_found_moves = test_board.get_valid_moves();
    assert_eq_lists(
        &test_board_found_moves, 
        &test_board_valid_moves
    );
}

#[test]
fn queen_movement_test() {
    let test_board = Board::from_fen(
        "8/8/1p2p3/1P2P3/8/8/1Q3q2/8 w - - 0 1"
    );
    let test_board_valid_moves = vec![
        boardmove("b2", "a1"),
        boardmove("b2", "b1"),
        boardmove("b2", "c1"),
        boardmove("b2", "a2"),
        boardmove("b2", "c2"),
        boardmove("b2", "a3"),
        boardmove("b2", "b3"),
        boardmove("b2", "c3"),
        boardmove("b2", "b4"),
        boardmove("b2", "d2"),
        boardmove("b2", "e2"),
        boardmove("b2", "f2"),
        boardmove("b2", "d4")
    ];
    let test_board_found_moves = test_board.get_valid_moves();
    assert_eq_lists(
        &test_board_found_moves, 
        &test_board_valid_moves
    );
}

#[test]
fn king_movement_test() {
    let test_board = Board::from_fen(
        "8/8/8/8/p7/P7/1Kq5/8 w - - 0 1"
    );
    let test_board_valid_moves = vec![
        boardmove("b2", "a1"),
        boardmove("b2", "b1"),
        boardmove("b2", "c1"),
        boardmove("b2", "a2"),
        boardmove("b2", "c2"),
        boardmove("b2", "b3"),
        boardmove("b2", "c3"),
    ];
    let test_board_found_moves = test_board.get_valid_moves();
    assert_eq_lists(
        &test_board_found_moves, 
        &test_board_valid_moves
    );
}

fn boardmove(s1: &str, s2: &str) -> BoardMove {
    return BoardMove::new(&BoardSquare::from(s1), &BoardSquare::from(s2));
}
fn assert_eq_lists<T>(a: &[T], b: &[T])
where
    T: PartialEq + Ord + fmt::Display
{
    let mut a: Vec<_> = a.iter().collect();
    let mut b: Vec<_> = b.iter().collect();
    a.sort();
    b.sort();

    if a != b {
        panic!("{} is not a permutation of {}", display_vec(a), display_vec(b));
    }
}
fn display_vec<T: PartialEq + Ord + fmt::Display>(vec: Vec<T>) -> String {
    return format!("[{}]", vec.iter().fold(String::new(), |acc, num| acc + &num.to_string() + ", "));
}
