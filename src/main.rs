use apotheosis::board::Board;
fn main() {
    let bb = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    println!("{}\n{:?}", bb, bb.get_valid_moves());
}
