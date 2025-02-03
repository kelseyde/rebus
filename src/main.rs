use crate::attacks::attacks;
use crate::board::Board;
use crate::consts::{Piece, Side};

mod board;
mod consts;
mod bits;
mod moves;
mod notation;
mod movegen;
mod attacks;
mod ray;
mod magics;
mod perft;

fn main() {
    // let board = Board::from_sfen(consts::STARTPOS.to_string()).unwrap();
    // let nodes = perft::perft(&board, 1);
    // println!("Nodes: {}", nodes);
    bits::print(attacks(12, Piece::Rook, Side::Sente, 0));
    println!("---");
    bits::print(attacks(0, Piece::Bishop, Side::Sente, 0));
}
