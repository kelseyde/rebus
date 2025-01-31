use arrayvec::ArrayVec;

use crate::consts::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Move(u16);

#[derive(Clone)]
pub struct MoveList {
    pub list: ArrayVec<Move, 600>,
    pub len: usize,
}

impl Move {

    pub const NONE: Move = Move(0);

    pub fn new(from: u8, to: u8, promote: bool) -> Self {
        assert!(from < 81 && to < 81);
        let mut value = (from as u16) | ((to as u16) << 6);
        if promote { value |= 1 << 12; }
        Self(value)
    }

    pub fn drop(piece: u8, to: u8) -> Self {
        assert!(piece < 15 && to < 81);
        let value = (piece as u16) | ((to as u16) << 6) | (1 << 13);
        Self(value)
    }

    pub fn src(self) -> Option<u8> {
        if self.is_drop() {
            None
        } else {
            Some((self.0 & 0b111111) as u8)
        }
    }

    pub fn dst(self) -> u8 {
        ((self.0 >> 6) & 0b111111) as u8
    }

    pub fn is_promo(self) -> bool {
        (self.0 & (1 << 12)) != 0
    }

    pub fn is_drop(self) -> bool {
        (self.0 & (1 << 13)) != 0
    }

    pub fn drop_piece(self) -> Option<Piece> {
        if self.is_drop() {
            Some(Piece::from((self.0 & 0b1111) as u8))
        } else {
            None
        }
    }

}

impl MoveList {

    pub fn new() -> Self {
        MoveList { list: ArrayVec::new(), len: 0 }
    }

    // pub fn add_move(&mut self, from: u8, to: u8, flag: MoveFlag) {
    //     self.list.push(Move::new(from, to, flag));
    //     self.len += 1;
    // }

    pub fn iter(&self) -> impl Iterator<Item = &Move> {
        self.list.iter().take(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

}