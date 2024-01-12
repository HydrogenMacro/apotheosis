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
fn pawn_movement_test() {
    let b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}", b);
    let moves = b.get_valid_moves();
    moves
        .into_iter()
        .for_each(|m| {
            println!("move from {} to {}", m.from_square(), m.dest_square())
        });
}
