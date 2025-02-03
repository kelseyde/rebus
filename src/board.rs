use crate::bits;
use crate::consts::Piece;
use crate::consts::{Side, Square};
use crate::moves::Move;

#[derive(Clone, Copy)]
pub struct Board {
    bb: [u128; Piece::COUNT + 2],          // bitboards for each piece type (0-13) and side (14-15)
    pcs: [Option<Piece>; Square::COUNT],   // piece type on each square
    hand: [Hand; 2],                       // pieces in hand for each side
    moves: u8,                             // number of moves made
    stm: Side                              // side to move
}

#[derive(Clone, Copy)]
pub struct Hand {
    pub pieces: [u8; 7]
}

impl Board {

    pub fn new() -> Board {
        Board {
            bb: [0; Piece::COUNT + 2],
            pcs: [None; Square::COUNT],
            hand: [Hand::new(), Hand::new()],
            stm: Side::Sente,
            moves: 0
        }
    }

    pub fn make(&mut self, mv: &Move) {

        if mv.is_drop() {
            let sq = mv.dst();
            let piece = mv.drop_piece().unwrap();
            self.drop_piece(self.stm, piece, sq);
        }
        else {
            let src = mv.src().unwrap();
            let dst = mv.dst();
            let src_piece = self.piece_at(src).unwrap();
            let dst_piece = if mv.is_promo() { src_piece.to_promo_piece().unwrap() } else { src_piece };
            let captured_piece = self.piece_at(dst);
            if captured_piece.is_some() {
                self.remove_piece(self.stm.flip(), captured_piece.unwrap(), dst);
                self.hand[self.stm.idx()].add(captured_piece.unwrap());
            }
            self.move_piece(self.stm, src_piece, dst_piece, src, dst);
        }
        self.moves += 1;
        self.stm = self.stm.flip();

    }

    #[inline]
    pub fn stm(&self) -> Side {
        self.stm
    }

    #[inline]
    pub fn moves(&self) -> u8 {
        self.moves
    }

    #[inline]
    pub fn hand(&self, side: Side) -> &Hand {
        &self.hand[side.idx()]
    }

    #[inline]
    pub fn pieces(&self, piece: Piece, side: Side) -> u128 {
        self.bb[piece.idx()]
            & self.bb[side.idx() + Piece::COUNT]
    }

    #[inline]
    pub fn pawns(&self, side: Side) -> u128 {
        self.pieces(Piece::Pawn, side)
    }

    #[inline]
    pub fn knights(&self, side: Side) -> u128 {
        self.pieces(Piece::Knight, side)
    }

    #[inline]
    pub fn lances(&self, side: Side) -> u128 {
        self.pieces(Piece::Lance, side)
    }

    #[inline]
    pub fn bishops(&self, side: Side) -> u128 {
        self.pieces(Piece::Bishop, side)
    }

    #[inline]
    pub fn rooks(&self, side: Side) -> u128 {
        self.pieces(Piece::Rook, side)
    }

    #[inline]
    pub fn silvers(&self, side: Side) -> u128 {
        self.pieces(Piece::Silver, side)
    }

    #[inline]
    pub fn golds(&self, side: Side) -> u128 {
        self.pieces(Piece::Gold, side)
    }

    #[inline]
    pub fn promoted_pawns(&self, side: Side) -> u128 {
        self.pieces(Piece::PromotedPawn, side)
    }

    #[inline]
    pub fn promoted_lances(&self, side: Side) -> u128 {
        self.pieces(Piece::PromotedLance, side)
    }

    #[inline]
    pub fn promoted_knights(&self, side: Side) -> u128 {
        self.pieces(Piece::PromotedKnight, side)
    }

    #[inline]
    pub fn promoted_silvers(&self, side: Side) -> u128 {
        self.pieces(Piece::PromotedSilver, side)
    }

    #[inline]
    pub fn promoted_bishops(&self, side: Side) -> u128 {
        self.pieces(Piece::PromotedBishop, side)
    }

    #[inline]
    pub fn promoted_rooks(&self, side: Side) -> u128 {
        self.pieces(Piece::PromotedRook, side)
    }

    #[inline]
    pub fn king_likes(&self, side: Side) -> u128 {
        self.king(side) | self.promoted_bishops(side) | self.promoted_rooks(side)
    }

    #[inline]
    pub fn bishop_likes(&self, side: Side) -> u128 {
        self.bishops(side) | self.promoted_bishops(side)
    }

    #[inline]
    pub fn rook_likes(&self, side: Side) -> u128 {
        self.rooks(side) | self.promoted_rooks(side)
    }

    #[inline]
    pub fn gold_likes(&self, side: Side) -> u128 {
        self.golds(side)
            | self.promoted_pawns(side)
            | self.promoted_lances(side)
            | self.promoted_knights(side)
            | self.promoted_silvers(side)
    }

    pub fn king(&self, side: Side) -> u128 {
        self.pieces(Piece::King, side)
    }

    #[inline]
    pub fn king_sq(&self, side: Side) -> u8 {
        bits::lsb(self.king(side))
    }

    #[inline]
    pub fn side(&self, side: Side) -> u128 {
        self.bb[side.idx() + Piece::COUNT]
    }

    #[inline]
    pub fn occ(&self) -> u128 {
        self.bb[Side::Sente.idx() + Piece::COUNT]
            | self.bb[Side::Gote.idx() + Piece::COUNT]
    }

    pub fn piece_at(&self, sq: u8) -> Option<Piece> {
        self.pcs[sq as usize]
    }

    pub fn side_at(&self, sq: u8) -> Option<Side> {
        if self.bb[Side::Sente.idx() + Piece::COUNT] & bits::bb(sq) != 0 { Some(Side::Sente) }
        else if self.bb[Side::Gote.idx() + Piece::COUNT] & bits::bb(sq) != 0 { Some(Side::Gote) }
        else { None }
    }

    pub fn drop_piece(&mut self, side: Side, piece: Piece, sq: u8) {
        let bb = bits::bb(sq);
        self.bb[piece.idx()] |= bb;
        self.bb[side.idx() + Piece::COUNT] |= bb;
        self.pcs[sq as usize] = Some(piece);
        self.hand[side.idx()].remove(piece);
        self.stm = side.flip();
    }

    pub fn move_piece(&mut self, side: Side, src_piece: Piece, dst_piece: Piece, src: u8, dst: u8) {
        let src_bb = bits::bb(src);
        let dst_bb = bits::bb(dst);
        self.bb[src_piece.idx()] ^= src_bb;
        self.bb[dst_piece.idx()] ^= dst_bb;
        self.bb[side.idx() + Piece::COUNT] ^= src_bb | dst_bb;
        self.pcs[src as usize] = None;
        self.pcs[dst as usize] = Some(dst_piece);
        self.stm = side.flip();
    }

    pub fn add_piece(&mut self, side: Side, piece: Piece, sq: u8) {
        let bb = bits::bb(sq);
        self.bb[piece.idx()] |= bb;
        self.bb[side.idx() + Piece::COUNT] |= bb;
        self.pcs[sq as usize] = Some(piece);
    }

    pub fn remove_piece(&mut self, side: Side, piece: Piece, sq: u8) {
        let bb = bits::bb(sq);
        self.bb[piece.idx()] ^= bb;
        self.bb[side.idx() + Piece::COUNT] ^= bb;
        self.pcs[sq as usize] = None;
    }

    pub fn set_stm(&mut self, side: Side) {
        self.stm = side;
    }

    pub fn set_moves(&mut self, moves: u8) {
        self.moves = moves;
    }

    pub fn set_hand(&mut self, side: Side, hand: Hand) {
        self.hand[side.idx()] = hand;
    }

}

impl Hand {
    pub fn new() -> Hand {
        Hand { pieces: [0; 7] }
    }

    #[inline]
    pub fn count(&self, piece: Piece) -> u8 {
        self.pieces[piece.idx()]
    }

    #[inline]
    pub fn add(&mut self, piece: Piece) {
        self.pieces[piece.idx()] += 1;
    }

    #[inline]
    pub fn remove(&mut self, piece: Piece) {
        self.pieces[piece.idx()] -= 1;
    }

    #[inline]
    pub fn has(&self, piece: Piece) -> bool {
        self.pieces[piece.idx()] > 0
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pieces.iter().all(|&x| x == 0)
    }

    pub fn clear(&mut self) {
        for x in self.pieces.iter_mut() {
            *x = 0;
        }
    }
}

