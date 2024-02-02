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
fn board_move_test() {
    const valid_move_tests = [
        // pins and en passant
        ("8/3p1p2/p2P1P2/Pp2BP1p/RP3PpP/2n2pP1/r4P2/k1K5 b - h3 0 1", [
            [
                ["^g4", "h3"],
                ["a2", "a3"],
                ["a2", "a4"],
                ["b5", "a4"],
                ["a2", "a4"],
            ],
            [
                ["a4", "a3"],
                ["a4", "a2"],
                ["e5", "d4"],
                ["e5", "c3"],
            ],
        ]),
        // castling
        ("r3k2r/Rp5p/pP5P/P7/1p6/pPp2p1p/PrP2P1P/R3K2R w KQq - 0 1", [
            [
                ["b2", "a1"],
                ["b2", "b1"],
                ["b2", "b3"],
                ["b2", "c2"],
                ["a8", "a7"],
                ["a8", "b8"],
                ["a8", "c8"],
                ["a8", "d8"],
                ["e8", "d8"],
                ["e8", "f8"],
                ["e8", "d7"],
                ["e8", "e7"],
                ["e8", "f7"],
                ["h8", "g8"],
                ["h8", "f8"],
                ["c", "bq"]
            ],
            [
                ["a1", "b1"],
                ["a1", "c1"],
                ["a1", "d1"],
                ["e1", "d1"],
                ["e1", "f1"],
                ["h1", "g1"],
                ["h1", "f1"],
                ["a7", "b7"],
                ["a7", "a6"],
                ["a7", "a8"],
                ["c", "wk"]
            ]
        ]),
    ];
}

fn boardmove(s1: &str, s2: &str) -> BoardMove {
    match s1.chars().nth(0).unwrap() {
        'c' => match s2 {
            "bk" => BoardMove::CASTLE_BK,
            "bq" => BoardMove::CASTLE_BQ,
            "wk" => BoardMove::CASTLE_WK,
            "wq" => BoardMove::CASTLE_WQ,
            _ => unreachable!()
        },
        '^' => BoardMove::new_as_en_passant(&BoardSquare::from(s1), &BoardSquare::from(s2)),
        _ => BoardMove::new(&BoardSquare::from(s1), &BoardSquare::from(s2))
    }
}