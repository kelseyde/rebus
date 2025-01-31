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
    if (sq >= 81) {
        panic!("Invalid square");
    }
    assert!(sq < 81);
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

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let bb = super::bb(0);
        super::print(bb);
        println!("{:b}", bb);
        let bb = super::bb(30);
        super::print(bb);
        println!("{:b}", bb);
    }

}