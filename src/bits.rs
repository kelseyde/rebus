use std::collections::HashMap;
use crate::consts::{Piece, Side, Square};

pub const ALL: u128 = (1 << 81) - 1;
pub const NONE: u128 = 0;

pub const RANK_I: u128 = 0x1FF;
pub const RANK_H: u128 = RANK_I << 9;
pub const RANK_G: u128 = RANK_I << 18;
pub const RANK_F: u128 = RANK_I << 27;
pub const RANK_E: u128 = RANK_I << 36;
pub const RANK_D: u128 = RANK_I << 45;
pub const RANK_C: u128 = RANK_I << 54;
pub const RANK_B: u128 = RANK_I << 63;
pub const RANK_A: u128 = RANK_I << 72;
pub const RANKS: [u128; 9] = [RANK_I, RANK_H, RANK_G, RANK_F, RANK_E, RANK_D, RANK_C, RANK_B, RANK_A];

pub const FILE_9: u128 = 0x1008040201008040201;
pub const FILE_8: u128 = FILE_9 << 1;
pub const FILE_7: u128 = FILE_9 << 2;
pub const FILE_6: u128 = FILE_9 << 3;
pub const FILE_5: u128 = FILE_9 << 4;
pub const FILE_4: u128 = FILE_9 << 5;
pub const FILE_3: u128 = FILE_9 << 6;
pub const FILE_2: u128 = FILE_9 << 7;
pub const FILE_1: u128 = FILE_9 << 8;
pub const FILES: [u128; 9] = [FILE_9, FILE_8, FILE_7, FILE_6, FILE_5, FILE_4, FILE_3, FILE_2, FILE_1];

pub const PROMO_ZONE: [u128; 2] = [RANK_A | RANK_B | RANK_C, RANK_G | RANK_H | RANK_I];
pub const MUST_PROMO_ZONE_PAWN: [u128; 2] = [RANK_A, RANK_I];
pub const MUST_PROMO_ZONE_KNIGHT: [u128; 2] = [RANK_A | RANK_B, RANK_H | RANK_I];

#[inline]
pub const fn pop(b: u128) -> u128 {
    b & (b - 1)
}

#[inline]
pub const fn lsb(b: u128) -> u8 {
    b.trailing_zeros() as u8
}

#[inline]
pub const fn count(b: u128) -> u8 {
    b.count_ones() as u8
}

#[inline]
pub const fn contains(bb: u128, sq: u8) -> bool {
    (bb >> sq) & 1 == 1
}

#[inline]
pub const fn merge(bb: u128) -> u64 {
    (bb & 0xFFFFFFFFFFFFFFFF) as u64 | ((bb >> 64) as u64)
}

#[inline]
pub const fn bb(sq: u8) -> u128 {
    1 << sq
}

#[inline]
pub const fn north(bb: u128) -> u128 {
    (bb << 9) & ALL
}

#[inline]
pub const fn south(bb: u128) -> u128 {
    (bb >> 9)
}

#[inline]
pub const fn east(bb: u128) -> u128 {
    (bb << 1) & !FILE_9
}

#[inline]
pub const fn west(bb: u128) -> u128 {
    (bb >> 1) & !FILE_1
}

#[inline]
pub const fn north_east(bb: u128) -> u128 {
    (bb << 10) & !FILE_9 & ALL
}

#[inline]
pub const fn north_west(bb: u128) -> u128 {
    (bb << 8) & !FILE_1 & ALL
}

#[inline]
pub const fn south_east(bb: u128) -> u128 {
    (bb >> 8) & !FILE_9
}

#[inline]
pub const fn south_west(bb: u128) -> u128 {
    (bb >> 10) & !FILE_1
}

#[inline]
pub fn file(sq: u8) -> u128 {
    FILES[Square::file(sq) as usize]
}

#[inline]
pub fn rank(sq: u8) -> u128 {
    RANKS[Square::rank(sq) as usize]
}

#[inline]
pub fn promo_zone(side: Side) -> u128 {
    PROMO_ZONE[side as usize]
}

pub const fn must_promo_zone(piece: Piece, side: Side) -> u128 {
    match piece {
        Piece::Pawn => MUST_PROMO_ZONE_PAWN[side as usize],
        Piece::Knight => MUST_PROMO_ZONE_KNIGHT[side as usize],
        _ => NONE,
    }
}

pub fn print(bb: u128) {
    for rank in (0..9).rev() {
        for file in 0..9 {
            let sq = Square::of(rank, file);
            let bit = (bb >> sq) & 1;
            print!("{} ", bit);
        }
        println!();
    }
}