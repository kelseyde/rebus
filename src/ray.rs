use crate::consts::Square;

pub fn between(mut from: u8, to: u8) -> u128 {
    if !Square::is_valid(from) || !Square::is_valid(to) || from == to {
        return 0;
    }
    let offset = direction(from, to);
    if offset == 0 {
        return 0;
    }
    let from: i8 = from as i8;
    let to: i8 = to as i8;
    let mut ray = 0;
    let mut sq = from.wrapping_add(offset);
    if sq < 0 || sq > 80 {
        return 0;
    }
    while Square::is_valid(sq as u8) && sq != to {
        ray |= 1 << sq;
        sq = sq.wrapping_add(offset);
    }
    ray
}

fn direction(from: u8, to: u8) -> i8 {
    let start_rank = Square::rank(from);
    let end_rank = Square::rank(to);
    let start_file = Square::file(from);
    let end_file = Square::file(to);
    if start_rank == end_rank {
        return if from > to { -1 } else { 1 };
    } else if start_file == end_file {
        return if from > to { -8 } else { 8 };
    } else if (start_rank as i8 - end_rank as i8).abs() == (start_file as i8 - end_file as i8).abs() {
        return if from > to {
            if (from as i8 - to as i8) % 9 == 0 { -9 } else { -7 }
        } else {
            if (to as i8 - from as i8) % 9 == 0 { 9 } else { 7 }
        };
    } else if start_rank as i8 + start_file as i8 == end_rank as i8 + end_file as i8 {
        return if from > to { -9 } else { 9 };
    }
    0
}