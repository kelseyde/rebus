use crate::bits;
use crate::consts::Side;

pub fn king(sq: u8) -> u128 {
    KING_ATTACKS[sq as usize]
}

pub fn pawns(bb: u128, side: Side) -> u128 {
    match side {
        Side::Sente => bits::north(bb),
        Side::Gote => bits::south(bb),
    }
}

pub fn knight(sq: u8, side: Side) -> u128 {
    KNIGHT_ATTACKS[side.idx()][sq as usize]
}

pub fn lance(sq: u8) -> u128 {
    //TODO
    0
}

pub fn bishop(sq: u8) -> u128 {
    //TODO
    0
}

pub fn rook(sq: u8) -> u128 {
    //TODO
    0
}

pub fn horse(sq: u8) -> u128 {
    king(sq) | bishop(sq)
}

pub fn dragon(sq: u8) -> u128 {
    king(sq) | rook(sq)
}

pub fn silver(sq: u8, side: Side) -> u128 {
    SILVER_ATTACKS[side.idx()][sq as usize]
}

pub fn gold(sq: u8, side: Side) -> u128 {
    GOLD_ATTACKS[side.idx()][sq as usize]
}

pub const KING_ATTACKS: [u128; 81] = [
    0x602, 0xe05, 0x1c0a, 0x3814, 0x7028, 0xe050, 0x1c0a0, 0x38140, 0x30080, 0xc0403, 0x1c0a07,
    0x38140e, 0x70281c, 0xe05038, 0x1c0a070, 0x38140e0, 0x70281c0, 0x6010180, 0x18080600,
    0x38140e00, 0x70281c00, 0xe0503800, 0x1c0a07000, 0x38140e000, 0x70281c000, 0xe05038000,
    0xc02030000, 0x30100c0000, 0x70281c0000, 0xe050380000, 0x1c0a0700000, 0x38140e00000,
    0x70281c00000, 0xe0503800000, 0x1c0a07000000, 0x180406000000, 0x602018000000, 0xe05038000000,
    0x1c0a070000000, 0x38140e0000000, 0x70281c0000000, 0xe050380000000, 0x1c0a0700000000,
    0x38140e00000000, 0x30080c00000000, 0xc0403000000000, 0x1c0a07000000000, 0x38140e000000000,
    0x70281c000000000, 0xe05038000000000, 0x1c0a070000000000, 0x38140e0000000000, 0x70281c0000000000,
    0x6010180000000000, 0x18080600000000000, 0x38140e00000000000, 0x70281c00000000000,
    0xe0503800000000000, 0x1c0a07000000000000, 0x38140e000000000000, 0x70281c000000000000,
    0xe05038000000000000, 0xc02030000000000000, 0x30100c0000000000000, 0x70281c0000000000000,
    0xe050380000000000000, 0x1c0a0700000000000000, 0x38140e00000000000000, 0x70281c00000000000000,
    0xe0503800000000000000, 0x1c0a07000000000000000, 0x180406000000000000000, 0x2018000000000000000,
    0x5038000000000000000, 0xa070000000000000000, 0x140e0000000000000000, 0x281c0000000000000000,
    0x50380000000000000000, 0xa0700000000000000000, 0x140e00000000000000000, 0x280c00000000000000000
];

pub const KNIGHT_ATTACKS: [[u128; 81]; 2] = [
    [0x80000, 0x140000, 0x280000, 0x500000, 0xa00000, 0x1400000, 0x2800000, 0x5000000, 0x2000000,
    0x10000000, 0x28000000, 0x50000000, 0xa0000000, 0x140000000, 0x280000000, 0x500000000,
    0xa00000000, 0x400000000, 0x2000000000, 0x5000000000, 0xa000000000, 0x14000000000, 0x28000000000,
    0x50000000000, 0xa0000000000, 0x140000000000, 0x80000000000, 0x400000000000, 0xa00000000000,
    0x1400000000000, 0x2800000000000, 0x5000000000000, 0xa000000000000, 0x14000000000000,
    0x28000000000000, 0x10000000000000, 0x80000000000000, 0x140000000000000, 0x280000000000000,
    0x500000000000000, 0xa00000000000000, 0x1400000000000000, 0x2800000000000000, 0x5000000000000000,
    0x2000000000000000, 0x10000000000000000, 0x28000000000000000, 0x50000000000000000,
    0xa0000000000000000, 0x140000000000000000, 0x280000000000000000, 0x500000000000000000,
    0xa00000000000000000, 0x400000000000000000, 0x2000000000000000000, 0x5000000000000000000,
    0xa000000000000000000, 0x14000000000000000000, 0x28000000000000000000, 0x50000000000000000000,
    0xa0000000000000000000, 0x140000000000000000000, 0x80000000000000000000, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
    [0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x2,
    0x5, 0xa, 0x14, 0x28, 0x50, 0xa0, 0x140, 0x80, 0x400, 0xa00, 0x1400, 0x2800, 0x5000, 0xa000,
    0x14000, 0x28000, 0x10000, 0x80000, 0x140000, 0x280000, 0x500000, 0xa00000, 0x1400000, 0x2800000,
    0x5000000, 0x2000000, 0x10000000, 0x28000000, 0x50000000, 0xa0000000, 0x140000000, 0x280000000,
    0x500000000, 0xa00000000, 0x400000000, 0x2000000000, 0x5000000000, 0xa000000000, 0x14000000000,
    0x28000000000, 0x50000000000, 0xa0000000000, 0x140000000000, 0x80000000000, 0x400000000000,
    0xa00000000000, 0x1400000000000, 0x2800000000000, 0x5000000000000, 0xa000000000000,
    0x14000000000000, 0x28000000000000, 0x10000000000000, 0x80000000000000, 0x140000000000000,
    0x280000000000000, 0x500000000000000, 0xa00000000000000, 0x1400000000000000, 0x2800000000000000,
    0x5000000000000000, 0x2000000000000000]
];

pub const SILVER_ATTACKS: [[u128; 81]; 2] = [
    [0x600, 0xe00, 0x1c00, 0x3800, 0x7000, 0xe000, 0x1c000, 0x38000, 0x30000, 0xc0002, 0x1c0005,
    0x38000a, 0x700014, 0xe00028, 0x1c00050, 0x38000a0, 0x7000140, 0x6000080, 0x18000400,
    0x38000a00, 0x70001400, 0xe0002800, 0x1c0005000, 0x38000a000, 0x700014000, 0xe00028000,
    0xc00010000, 0x3000080000, 0x7000140000, 0xe000280000, 0x1c000500000, 0x38000a00000,
    0x70001400000, 0xe0002800000, 0x1c0005000000, 0x180002000000, 0x600010000000, 0xe00028000000,
    0x1c00050000000, 0x38000a0000000, 0x7000140000000, 0xe000280000000, 0x1c000500000000,
    0x38000a00000000, 0x30000400000000, 0xc0002000000000, 0x1c0005000000000, 0x38000a000000000,
    0x700014000000000, 0xe00028000000000, 0x1c00050000000000, 0x38000a0000000000,
    0x7000140000000000, 0x6000080000000000, 0x18000400000000000, 0x38000a00000000000,
    0x70001400000000000, 0xe0002800000000000, 0x1c0005000000000000, 0x38000a000000000000,
    0x700014000000000000, 0xe00028000000000000, 0xc00010000000000000, 0x3000080000000000000,
    0x7000140000000000000, 0xe000280000000000000, 0x1c000500000000000000, 0x38000a00000000000000,
    0x70001400000000000000, 0xe0002800000000000000, 0x1c0005000000000000000,
    0x180002000000000000000, 0x10000000000000000, 0x28000000000000000, 0x50000000000000000,
    0xa0000000000000000, 0x140000000000000000, 0x280000000000000000, 0x500000000000000000,
    0xa00000000000000000, 0x400000000000000000],
    [0x400, 0xa00, 0x1400, 0x2800, 0x5000, 0xa000, 0x14000, 0x28000, 0x10000, 0x80003, 0x140007,
    0x28000e, 0x50001c, 0xa00038, 0x1400070, 0x28000e0, 0x50001c0, 0x2000180, 0x10000600,
    0x28000e00, 0x50001c00, 0xa0003800, 0x140007000, 0x28000e000, 0x50001c000, 0xa00038000,
    0x400030000, 0x20000c0000, 0x50001c0000, 0xa000380000, 0x14000700000, 0x28000e00000,
    0x50001c00000, 0xa0003800000, 0x140007000000, 0x80006000000, 0x400018000000, 0xa00038000000,
    0x1400070000000, 0x28000e0000000, 0x50001c0000000, 0xa000380000000, 0x14000700000000,
    0x28000e00000000, 0x10000c00000000, 0x80003000000000, 0x140007000000000, 0x28000e000000000,
    0x50001c000000000, 0xa00038000000000, 0x1400070000000000, 0x28000e0000000000,
    0x50001c0000000000, 0x2000180000000000, 0x10000600000000000, 0x28000e00000000000,
    0x50001c00000000000, 0xa0003800000000000, 0x140007000000000000, 0x28000e000000000000,
    0x50001c000000000000, 0xa00038000000000000, 0x400030000000000000, 0x20000c0000000000000,
    0x50001c0000000000000, 0xa000380000000000000, 0x14000700000000000000, 0x28000e00000000000000,
    0x50001c00000000000000, 0xa0003800000000000000, 0x140007000000000000000,
    0x80006000000000000000, 0x18000000000000000, 0x38000000000000000, 0x70000000000000000,
    0xe0000000000000000, 0x1c0000000000000000, 0x380000000000000000, 0x700000000000000000,
    0xe00000000000000000, 0xc00000000000000000]
];

pub const GOLD_ATTACKS: [[u128; 81]; 2] = [
    [0x602, 0xe05, 0x1c0a, 0x3814, 0x7028, 0xe050, 0x1c0a0, 0x38140, 0x30080, 0xc0401, 0x1c0a02,
    0x381404, 0x702808, 0xe05010, 0x1c0a020, 0x3814040, 0x7028080, 0x6010100, 0x18080200, 0x38140400,
    0x70280800, 0xe0501000, 0x1c0a02000, 0x381404000, 0x702808000, 0xe05010000, 0xc02020000,
    0x3010040000, 0x7028080000, 0xe050100000, 0x1c0a0200000, 0x38140400000, 0x70280800000,
    0xe0501000000, 0x1c0a02000000, 0x180404000000, 0x602008000000, 0xe05010000000, 0x1c0a020000000,
    0x3814040000000, 0x7028080000000, 0xe050100000000, 0x1c0a0200000000, 0x38140400000000,
    0x30080800000000, 0xc0401000000000, 0x1c0a02000000000, 0x381404000000000, 0x702808000000000,
    0xe05010000000000, 0x1c0a020000000000, 0x3814040000000000, 0x7028080000000000, 0x6010100000000000,
    0x18080200000000000, 0x38140400000000000, 0x70280800000000000, 0xe0501000000000000,
    0x1c0a02000000000000, 0x381404000000000000, 0x702808000000000000, 0xe05010000000000000,
    0xc02020000000000000, 0x3010040000000000000, 0x7028080000000000000, 0xe050100000000000000,
    0x1c0a0200000000000000, 0x38140400000000000000, 0x70280800000000000000, 0xe0501000000000000000,
    0x1c0a02000000000000000, 0x180404000000000000000, 0x2008000000000000000, 0x5010000000000000000,
    0xa020000000000000000, 0x14040000000000000000, 0x28080000000000000000, 0x50100000000000000000,
    0xa0200000000000000000, 0x140400000000000000000, 0x280800000000000000000],
    [0x202, 0x405, 0x80a, 0x1014, 0x2028, 0x4050, 0x80a0, 0x10140, 0x20080, 0x40403, 0x80a07,
    0x10140e, 0x20281c, 0x405038, 0x80a070, 0x10140e0, 0x20281c0, 0x4010180, 0x8080600, 0x10140e00,
    0x20281c00, 0x40503800, 0x80a07000, 0x10140e000, 0x20281c000, 0x405038000, 0x802030000,
    0x10100c0000, 0x20281c0000, 0x4050380000, 0x80a0700000, 0x10140e00000, 0x20281c00000,
    0x40503800000, 0x80a07000000, 0x100406000000, 0x202018000000, 0x405038000000, 0x80a070000000,
    0x10140e0000000, 0x20281c0000000, 0x4050380000000, 0x80a0700000000, 0x10140e00000000,
    0x20080c00000000, 0x40403000000000, 0x80a07000000000, 0x10140e000000000, 0x20281c000000000,
    0x405038000000000, 0x80a070000000000, 0x10140e0000000000, 0x20281c0000000000, 0x4010180000000000,
    0x8080600000000000, 0x10140e00000000000, 0x20281c00000000000, 0x40503800000000000,
    0x80a07000000000000, 0x10140e000000000000, 0x20281c000000000000, 0x405038000000000000,
    0x802030000000000000, 0x10100c0000000000000, 0x20281c0000000000000, 0x4050380000000000000,
    0x80a0700000000000000, 0x10140e00000000000000, 0x20281c00000000000000, 0x40503800000000000000,
    0x80a07000000000000000, 0x100406000000000000000, 0x2018000000000000000, 0x5038000000000000000,
    0xa070000000000000000, 0x140e0000000000000000, 0x281c0000000000000000, 0x50380000000000000000,
    0xa0700000000000000000, 0x140e00000000000000000, 0x280c00000000000000000]
];

#[cfg(test)]
mod test {
    use crate::{attacks, bits};
    use crate::bits::bb;
    use crate::consts::Side;

    #[test]
    pub fn knight() {

        for sq in 0..81 {
            println!("generating for square {}", sq);
            bits::print(bits::bb(sq));
            println!();
            let mut bb = attacks::knight(sq, Side::Sente);
            bits::print(bb);
            println!();
            bb = attacks::knight(sq, Side::Gote);
            bits::print(bb);
            println!();
        }
    }

    #[test]
    pub fn king() {
        for sq in 0..81 {
            println!("generating for square {}", sq);
            bits::print(bits::bb(sq));
            println!();
            let bb = attacks::king(sq);
            bits::print(bb);
            println!();
        }
    }

    #[test]
    pub fn gold() {
        for sq in 0..81 {
            println!("{:#x},", attacks::gold(sq, Side::Gote));
        }
    }

    #[test]
    pub fn silver() {
        for sq in 0..81 {
            print!("{:#x}, ", attacks::silver(sq, Side::Gote));
        }
    }

}