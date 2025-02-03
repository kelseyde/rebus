use crate::board::Board;

pub fn perft(board: &Board, depth: u8) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;

    let moves = board.generate_moves();
    for i in 0..moves.len() {
        let mut board = board.clone();
        board.make(&moves.get(i).unwrap());
        nodes += perft(&board, depth - 1);
    }

    nodes
}