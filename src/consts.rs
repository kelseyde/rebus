#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Lance,
    Knight,
    Silver,
    Gold,
    Bishop,
    Rook,
    PromotedPawn,
    PromotedLance,
    PromotedKnight,
    PromotedSilver,
    PromotedBishop,
    PromotedRook,
    King
}

impl Piece {
    pub const COUNT: usize = 14;        // All piece types (including promos and king)

    pub fn idx(&self) -> usize {
        *self as usize
    }

    pub fn to_promo_piece(&self) -> Option<Piece> {
        match self {
            Piece::Pawn =>   Some(Piece::PromotedPawn),
            Piece::Lance =>  Some(Piece::PromotedLance),
            Piece::Knight => Some(Piece::PromotedKnight),
            Piece::Silver => Some(Piece::PromotedSilver),
            Piece::Bishop => Some(Piece::PromotedBishop),
            Piece::Rook =>   Some(Piece::PromotedRook),
            _ => None,
        }
    }
}

impl From<u8> for Piece {
    fn from(value: u8) -> Self {
        match value {
            0 => Piece::Pawn,
            1 => Piece::Lance,
            2 => Piece::Knight,
            3 => Piece::Silver,
            4 => Piece::Gold,
            5 => Piece::Bishop,
            6 => Piece::Rook,
            7 => Piece::PromotedPawn,
            8 => Piece::PromotedLance,
            9 => Piece::PromotedKnight,
            10 => Piece::PromotedSilver,
            11 => Piece::PromotedBishop,
            12 => Piece::PromotedRook,
            13 => Piece::King,
            _ => panic!("Invalid piece index: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    First,
    Second,
}

impl Side {

    pub fn flip(&self) -> Side {
        match self {
            Side::First => Side::Second,
            Side::Second => Side::First,
        }
    }

    pub const fn idx(&self) -> usize {
        *self as usize
    }

}

pub struct Square {}
impl Square {
    pub const COUNT: usize = 81;
}