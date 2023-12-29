use apotheosis::Board;
fn main() {
    let bb = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    // println!("{:?}", bb);
    bb.print();
    let vm = bb.get_valid_moves();
    println!("{:?} with {} moves", vm, vm.len());
}