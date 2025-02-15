use crate::attacks::attacks;
use crate::consts::Side;

mod board;
mod consts;
mod bits;
mod moves;
mod notation;
mod movegen;
mod attacks;
mod ray;
mod perft;
mod magics;
mod generator;

fn main() {

    // generator::find_and_print_all_magics(&magics2::ROOK_DELTAS, "ROOK");
    // generator::find_and_print_all_magics(&magics2::BISHOP_DELTAS, "BISHOP");

    let attacks = attacks::rook(0, 0);
    bits::print(attacks);

    let attacks = attacks::bishop(0, 0);
    bits::print(attacks);

    let attacks = attacks::lance(0, Side::Sente, 0);
    bits::print(attacks);

    let moves = magics::attack_mask(&magics::LANCE_DELTAS[Side::Sente.idx()], 0);
    bits::print(moves);

    let attacks = attacks::rook(63, 0);
    bits::print(attacks);

    let attacks = attacks::bishop(63, 0);
    bits::print(attacks);

    let attacks = attacks::lance(63, Side::Gote, 0);
    bits::print(attacks);

    let moves = magics::attack_mask(&magics::LANCE_DELTAS[Side::Gote.idx()], 63);
    bits::print(moves);

    let attacks = attacks::rook(27, 0);
    bits::print(attacks);

    let attacks = attacks::bishop(27, 0);
    bits::print(attacks);

    let attacks = attacks::lance(27, Side::Sente, 0);
    bits::print(attacks);
    let moves = magics::attack_mask(&magics::LANCE_DELTAS[Side::Sente.idx()], 27);
    bits::print(moves);



}
