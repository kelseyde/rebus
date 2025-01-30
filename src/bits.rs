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