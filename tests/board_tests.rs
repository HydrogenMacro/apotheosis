use apotheosis::board::Board;

#[test]
fn pawn_movement_test() {
    let a7_pawn_board = Board::from_fen("8/p7/8/8/8/8/8/8 w KQkq - 0 1");
    assert_eq!(a7_pawn_board.get_valid_moves(), vec![(sq("a7"), sq("a6")), (sq("a7"), sq("a5"))]);
    let a6_pawn_board = Board::from_fen("8/8/p6/8/8/8/8/8 w KQkq - 0 1");
    assert_eq!(a7_pawn_board.get_valid_moves(), vec![(sq("a7"), sq("a6"))]);
}

#[test]
fn knight_movement_test() {
    let d5_knight_board = Board::from_fen("8/8/8/3n4/8/8/8/8 w KQkq - 0 1");
    assert_eq!(d5_knight_board.get_valid_moves(), vec![
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
        }
        let row_value = row.to_digit(10).unwrap() - 1;
        return col_value as i8 + row_value as i8;
    }
    panic!("board_square should only have 2 characters");
}