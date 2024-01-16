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
    assert_eq_lists(
        &pawn_test_board_found_moves, 
        &pawn_test_board_valid_moves
    );
}

fn boardmove(s1: &str, s2: &str) -> BoardMove {
    return BoardMove::new(&BoardSquare::from(s1), &BoardSquare::from(s2));
}
fn assert_eq_lists<T>(a: &[T], b: &[T])
where
    T: PartialEq + Ord
{
    let mut a: Vec<_> = a.iter().collect();
    let mut b: Vec<_> = b.iter().collect();
    a.sort();
    b.sort();

    if a != b {
        panic!("{} is not a permutation of {}", display_vec(a), display_vec(b));
    }
}
fn display_vec<T: fmt::Display>(vec: Vec<T>) -> String {
    return format!("[{}]", vec.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "));
}
