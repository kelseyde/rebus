use crate::{attacks, bits};
use crate::board::Board;
use crate::consts::Side;
use crate::moves::{Move, MoveList};

impl Board {

    pub fn generate_moves(board: &Board) -> MoveList {

        let mut moves = MoveList::new();
        let stm = board.stm();



        moves

    }

    fn generate_king_moves(move_list: &MoveList, board: &Board) {
        let stm = board.stm();
        let king = board.king(stm);
        let src = bits::lsb(king);
        let mut attacks = attacks::king(src) & !board.side(stm);
        while attacks != 0 {
            let dst = bits::lsb(attacks);
            move_list.add(Move::new(src, dst, false));
            attacks = bits::pop(attacks);
        }
    }

}