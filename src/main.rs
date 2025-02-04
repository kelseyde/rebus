use crate::attacks::attacks;
use crate::board::Board;
use crate::consts::{Piece, Side, Square};
use crate::magics::sliding_attacks;

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

fn attack_mask(deltas: &[i8], sq: u8) -> u128 {
    let occ = bits::NONE;
    let mut bb = sliding_attacks(deltas, sq, occ);
    let file = Square::file(sq);
    let rank = Square::rank(sq);
    // if file != 8 {
    //     bb &= !bits::FILE_1;
    // }
    // if file != 0 {
    //     bb &= !bits::FILE_9;
    // }
    // if rank != 8 {
    //     bb &= !bits::RANK_A;
    // }
    // if rank != 0 {
    //     bb &= !bits::RANK_I;
    // }
    bb
}

pub struct Bitboard {
    v: [u64; 2],
}


impl Bitboard {
    pub fn to_u128(&self) -> u128 {
        self.v[0] as u128 | (self.v[1] as u128) << 64
    }
}

fn main() {
    // let board = Board::from_sfen(consts::STARTPOS.to_string()).unwrap();
    // let nodes = perft::perft(&board, 1);
    // println!("Nodes: {}", nodes);

    println!("---");
    bits::print(attacks(0, Piece::Rook, Side::Sente, 0));
    println!("---");
    bits::print(attacks(4, Piece::Rook, Side::Sente, 0));
    println!("---");
    bits::print(attacks(8, Piece::Rook, Side::Sente, 0));
    println!("---");
    bits::print(attacks(20, Piece::Rook, Side::Sente, 0));
    println!("---");
    bits::print(attacks(67, Piece::Rook, Side::Sente, 0));
    println!("---");
    bits::print(attacks(79, Piece::Rook, Side::Sente, 0));
    println!("---");

}
