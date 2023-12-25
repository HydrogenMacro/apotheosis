fn main() {
    //let b = Board::from_raw([BoardSquare::WPawn; 64], White, [[true, true], [true, true]], 0, None);
    let bb = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{:?}", bb);
    //bb.print();
}