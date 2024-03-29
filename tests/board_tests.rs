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
    let valid_move_tests = [
        // pins and en passant
        ("8/3p1p2/p2P1P2/Pp2BP1p/RP3PpP/2n2pP1/r4P2/k1K5 b - h3 0 1", [
                vec![
                    ["^g4", "h3"],
                    ["a2", "a3"],
                    ["a2", "a4"],
                    ["b5", "a4"],
                    ["a2", "a4"],
                ],
                vec![
                    ["a4", "a3"],
                    ["a4", "a2"],
                    ["e5", "d4"],
                    ["e5", "c3"],
                ]
            ]
        ),
        // castling
        ("r3k2r/Rp5p/pP5P/P7/1p6/pPp2p1p/PrP2P1P/R3K2R w KQq - 0 1", [
                vec![
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
                vec![
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
            ]
        )
    ];

    for (test_fen, test_moves_raw) in valid_move_tests {
        println!("TESTING {}", test_fen);
        let found_moves = Board::from_fen(test_fen).get_board_info().valid_moves;
        for color in [BLACK, WHITE] {
            let test_moves: Vec<BoardMove> = test_moves_raw[color as usize].iter().map(|[s1, s2]| boardmove(s1, s2)).collect();
            assert_consists_of_same_moves(found_moves[color as usize].clone(), test_moves);
        }
    }
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
        '^' => BoardMove::new_as_en_passant(&BoardSquare::from(&s1[1..]), &BoardSquare::from(s2)),
        _ => BoardMove::new(&BoardSquare::from(s1), &BoardSquare::from(s2))
    }
}

fn assert_consists_of_same_moves(v1: Vec<BoardMove>, v2: Vec<BoardMove>) {
    let s1: HashSet<_> = v1.into_iter().collect();
    let s2: HashSet<_> = v2.into_iter().collect();
    let sdiff: Vec<&BoardMove> = s1.symmetric_difference(&s2).collect();
    for diff in sdiff.iter() {
        println!("difference: {}", diff);
    }
    assert_eq!(sdiff, [] as [&BoardMove; 0]);
}