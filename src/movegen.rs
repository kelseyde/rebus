use crate::{attacks, bits};
use crate::board::Board;
use crate::consts::{Piece, Side};
use crate::moves::{Move, MoveList};

impl Board {

    pub fn generate_moves(&self) -> MoveList {

        let mut moves = MoveList::new();
        let occ = self.occ();

        self.generate_pawns(&mut moves);
        self.generate(&mut moves, Piece::Lance, occ);
        self.generate(&mut moves, Piece::Knight, occ);
        self.generate(&mut moves, Piece::Silver, occ);
        self.generate(&mut moves, Piece::Gold, occ);
        self.generate(&mut moves, Piece::Bishop, occ);
        self.generate(&mut moves, Piece::Rook, occ);

        moves

    }

    fn generate(&self, move_list: &mut MoveList, piece: Piece, occ: u128) {
        let stm = self.stm();
        let friendlies = self.side(stm);
        let promo_zone = bits::promo_zone(stm);
        let must_promo_zone = bits::must_promo_zone(piece, stm);
        let mut pieces = self.pieces(piece, stm);
        while pieces != 0 {
            let src = bits::lsb(pieces);
            let mut attacks = attacks::attacks(src, piece, stm, occ) & !friendlies;
            while attacks != 0 {
                let dst = bits::lsb(attacks);
                self.add_moves(move_list, src, dst, promo_zone, must_promo_zone);
                attacks = bits::pop(attacks);
            }
            pieces = bits::pop(pieces);
        }
    }

    fn generate_pawns(&self, move_list: &mut MoveList) {
        let stm = self.stm();
        let pawns = self.pawns(stm);
        let promo_zone = bits::promo_zone(stm);
        let must_promo_zone = bits::must_promo_zone(Piece::Pawn, stm);
        let mut attacks = attacks::pawns(pawns, stm);
        while attacks != 0 {
            let dst = bits::lsb(attacks);
            let src = match stm { Side::Sente => dst - 9, Side::Gote => dst + 9 };
            self.add_moves(move_list, src, dst, promo_zone, must_promo_zone);
            attacks = bits::pop(attacks);
        }
    }

    fn add_moves(&self, move_list: &mut MoveList, src: u8, dst: u8, promo_zone: u128, must_promo_zone: u128) {
        let can_promo = bits::contains(promo_zone, dst);
        let must_promo = bits::contains(must_promo_zone, dst);
        if can_promo || must_promo {
            move_list.push(Move::new(src, dst, true));
        }
        if !must_promo {
            move_list.push(Move::new(src, dst, false));
        }
    }

    pub fn attackers_to(&self, sq: u8, side: Side) -> u128 {
        let occ = self.occ();
        let mut attackers = 0;
        attackers |= attacks::king(sq)                      & self.king_likes(side.flip());
        attackers |= attacks::pawn(sq, side.flip())         & self.pawns(side.flip());
        attackers |= attacks::knight(sq, side.flip())       & self.knights(side.flip());
        attackers |= attacks::silver(sq, side.flip())       & self.silvers(side.flip());
        attackers |= attacks::gold(sq, side.flip())         & self.gold_likes(side.flip());
        attackers |= attacks::lance(sq, side.flip(), occ)   & self.lances(side.flip());
        attackers |= attacks::bishop(sq, occ)               & self.bishop_likes(side.flip());
        attackers |= attacks::rook(sq, occ)                 & self.rook_likes(side.flip());
        attackers
    }

    pub fn is_check(&self) -> bool {
        let king_sq = self.king_sq(self.stm());
        self.is_attacked(king_sq, self.stm().flip())
    }

    pub fn is_attacked(&self, sq: u8, side: Side) -> bool {
        let occ = self.occ();
        attacks::king(sq)                           & self.king_likes(side.flip()) != 0 ||
            attacks::pawn(sq, side.flip())          & self.pawns(side.flip()) != 0 ||
            attacks::knight(sq, side.flip())        & self.knights(side.flip()) != 0 ||
            attacks::silver(sq, side.flip())        & self.silvers(side.flip()) != 0 ||
            attacks::gold(sq, side.flip())          & self.gold_likes(side.flip()) != 0 ||
            attacks::lance(sq, side.flip(), occ)    & self.lances(side.flip()) != 0 ||
            attacks::bishop(sq, occ)                & self.bishop_likes(side.flip()) != 0 ||
            attacks::rook(sq, occ)                  & self.rook_likes(side.flip()) != 0
    }

}