use crate::consts::Square;

pub const ALL: u128 = !0;
pub const NONE: u128 = 0;

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
pub const fn bb(sq: u8) -> u128 {
    1 << sq
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