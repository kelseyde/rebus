use crate::bits;
use crate::consts::Square;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BISHOP_MAGICS_TABLE: MagicTable<'static> = MagicTable::new(
        BISHOP_ATTACK_TABLE_NUM,
        &BISHOP_SHIFT_BITS,
        &BISHOP_MAGICS,
        &BISHOP_DELTAS,
    );

    pub static ref ROOK_MAGICS_TABLE: MagicTable<'static> = MagicTable::new(
        ROOK_ATTACK_TABLE_NUM,
        &ROOK_SHIFT_BITS,
        &ROOK_MAGICS,
        &ROOK_DELTAS,
    );
}

#[derive(Debug)]
pub struct Magic<'a> {
    mask: u128,
    magic: u64,
    attacks: &'a [u128],
    shift: u32,
}

pub struct MagicTable<'a> {
    magics: [Magic<'a>; Square::COUNT],
    _attacks: Vec<u128>,
}

impl<'a> Magic<'a> {

    fn attack_mask(deltas: &[i8], sq: u8) -> u128 {
        let occ = bits::NONE;
        let mut bb = sliding_attacks(deltas, sq, occ);
        let file = bits::file(sq);
        let rank = bits::rank(sq);
        if file != bits::FILE_1 {
            bb &= !bits::FILE_1;
        }
        if file != bits::FILE_9 {
            bb &= !bits::FILE_9;
        }
        if rank != bits::RANK_I {
            bb &= !bits::RANK_I;
        }
        if rank != bits::RANK_A {
            bb &= !bits::RANK_A;
        }
        bb
    }

    fn index_to_occ(index: u32, bit_count: u32, mask: u128) -> u128 {
        let mut ret = bits::NONE;
        let mut mask = mask;
        for i in 0..bit_count {
            let sq = bits::lsb(mask);
            if (index & (1 << i)) != 0 {
                ret |= bits::bb(sq);
            }
            mask = bits::pop(mask);
        }
        ret
    }

    fn occ_to_index(occ: u128, magic: u64, shift: u32) -> usize {
        let merged = bits::merge(occ);
        (merged.wrapping_mul(magic) >> shift) as usize
    }

    pub fn attack(&self, occ: u128) -> u128 {
        unsafe {
            let index = Self::occ_to_index(occ, self.magic, self.shift);
            *self.attacks.get_unchecked(index)
        }
    }

    fn pseudo_attack(&self) -> u128 {
        debug_assert!(!self.attacks.is_empty());
        unsafe { *self.attacks.get_unchecked(0) }
    }
}

impl<'a> MagicTable<'a> {

    fn new(
        table_num: usize,
        shifts: &[i8; Square::COUNT],
        magic_nums: &[u64; Square::COUNT],
        deltas: &[i8],
    ) -> MagicTable<'a> {
        let mut attacks = vec![bits::NONE; table_num];
        let mut magics = std::mem::MaybeUninit::<[Magic<'a>; Square::COUNT]>::uninit();
        let mut count = 0;
        for sq in 0..81 {
            let mask = Magic::attack_mask(deltas, sq);
            let slice_attacks = unsafe {
                let ptr = attacks.as_mut_ptr().add(count);
                std::slice::from_raw_parts_mut(ptr, 1 << (64 - shifts[sq as usize]))
            };
            for index in 0..(1 << mask.count_ones()) {
                let occ = Magic::index_to_occ(index, mask.count_ones(), mask);
                let idx = Magic::occ_to_index(occ, magic_nums[sq as usize], shifts[sq as usize] as u32);
                slice_attacks[idx] = sliding_attacks(deltas, sq, occ);
            }
            count += slice_attacks.len();
            let tmp_magic: Magic = Magic {
                mask: Magic::attack_mask(deltas, sq),
                magic: magic_nums[sq as usize],
                attacks: slice_attacks,
                shift: shifts[sq as usize] as u32,
            };
            unsafe {
                (magics.as_mut_ptr() as *mut Magic).add(sq as usize).write(tmp_magic);
            }
        }
        debug_assert_eq!(table_num, count);
        MagicTable {
            magics: unsafe { magics.assume_init() },
            _attacks: attacks,
        }
    }

    pub fn magic(&self, sq: u8) -> &Magic {
        debug_assert!(0 <= sq && (sq as usize) < self.magics.len());
        unsafe { self.magics.get_unchecked(sq as usize) }
    }
}


pub fn sliding_attacks(deltas: &[i8], sq: u8, occupied: u128) -> u128 {
    let mut bb: u128 = 0;

    for &delta in deltas {
        let mut sq_prev = sq;
        let mut sq_opt = checked_add(sq, delta);

        while let Some(sq_tmp) = sq_opt {
            if (Square::file(sq_prev) as i8 - Square::file(sq_tmp) as i8).abs() <= 1
                && (Square::rank(sq_prev) as i8 - Square::rank(sq_tmp) as i8).abs() <= 1
            {
                bb |= 1 << sq_tmp;

                if (occupied & (1 << sq_tmp)) != 0 {
                    break;
                }

                sq_prev = sq_tmp;
                sq_opt = checked_add(sq_tmp, delta);
            } else {
                break;
            }
        }
    }

    bb
}

#[inline]
pub fn checked_add(sq: u8, delta: i8) -> Option<u8> {
    let res = sq as i16 + delta as i16;
    if Square::is_valid(res as u8) {
        Some(res as u8)
    } else {
        None
    }
}

const BISHOP_DELTAS: [i8; 4] = [Square::DELTA_NE, Square::DELTA_SE, Square::DELTA_SW, Square::DELTA_NW];
const ROOK_DELTAS: [i8; 4] = [Square::DELTA_N, Square::DELTA_E, Square::DELTA_S, Square::DELTA_W];

const BISHOP_ATTACK_TABLE_NUM: usize = 20224;

const ROOK_ATTACK_TABLE_NUM: usize = 512_000;

const ROOK_SHIFT_BITS: [i8; Square::COUNT] = [
    50, 51, 51, 51, 51, 51, 51, 51, 50,
    51, 52, 52, 52, 52, 52, 52, 52, 50,
    51, 52, 52, 52, 52, 52, 52, 52, 51,
    51, 52, 52, 52, 52, 52, 52, 52, 51,
    51, 52, 52, 52, 52, 52, 52, 52, 51,
    51, 52, 52, 52, 52, 52, 52, 52, 50,
    51, 52, 52, 52, 52, 52, 52, 52, 51,
    51, 52, 52, 52, 52, 52, 52, 52, 51,
    50, 51, 51, 51, 51, 51, 51, 51, 50,
];

const BISHOP_SHIFT_BITS: [i8; Square::COUNT] = [
    57, 58, 58, 58, 58, 58, 58, 58, 57,
    58, 58, 58, 58, 58, 58, 58, 58, 58,
    58, 58, 56, 56, 56, 56, 56, 58, 58,
    58, 58, 56, 54, 54, 54, 56, 58, 58,
    58, 58, 56, 54, 52, 54, 56, 58, 58,
    58, 58, 56, 54, 54, 54, 56, 58, 58,
    58, 58, 56, 56, 56, 56, 56, 58, 58,
    58, 58, 58, 58, 58, 58, 58, 58, 58,
    57, 58, 58, 58, 58, 58, 58, 58, 57,
];

const BISHOP_MAGICS: [u64; Square::COUNT] = [
    0x2010_1042_c820_0428, 0x0000_8402_4038_0102, 0x8008_00c0_1810_8251,
    0x0082_4280_1030_1000, 0x0481_0082_0100_0040, 0x8081_0204_2088_0800,
    0x0000_8042_2211_0000, 0x0000_e283_0140_0850, 0x2010_2214_2080_0810,
    0x2600_0100_2880_1824, 0x0008_0481_0210_2002, 0x4000_2481_0024_0402,
    0x4920_0200_428a_2108, 0x0000_4609_0402_0844, 0x2001_4010_2083_0200,
    0x0000_0010_0900_8120, 0x4804_0640_0820_8004, 0x4406_0002_4030_0ca0,
    0x0222_0014_0080_3220, 0x0226_0684_0018_2094, 0x9520_8402_010d_0104,
    0x4000_8075_0010_8102, 0xc000_2000_8050_0500, 0x5211_0003_0403_8020,
    0x1108_1001_8040_0820, 0x1000_1280_a8a2_1040, 0x1000_0480_9408_a210,
    0x0202_3000_0204_1112, 0x0404_0a80_0046_0408, 0x0204_0200_2104_0201,
    0x0008_1200_1318_0404, 0xa284_0080_0d02_0104, 0x200c_2010_0060_4080,
    0x1082_0040_0010_9408, 0x1000_21c0_0c41_0408, 0x8808_2090_5004_c801,
    0x1054_0640_8000_4120, 0x030c_0a02_2400_1030, 0x0300_0601_0004_0821,
    0x0512_0080_1020_c006, 0x2100_0400_4280_2801, 0x0481_0008_2040_1002,
    0x4040_8a04_5000_0801, 0x0081_0104_2000_00a2, 0x0281_1021_0210_8408,
    0x0804_0200_4028_0021, 0x2420_4012_0022_0040, 0x0800_1014_4080_c402,
    0x0080_1044_0080_0002, 0x1009_0480_8040_0081, 0x1000_8200_0201_008c,
    0x0010_0010_0808_0009, 0x02a5_006b_8008_0004, 0xc628_8018_200c_2884,
    0x1081_0010_4200_a000, 0x0141_0020_3081_4048, 0x0200_2040_8001_0808,
    0x0200_0040_1392_2002, 0x2200_0000_2005_0815, 0x2011_0104_0004_0800,
    0x1020_0400_0422_0200, 0x0944_0201_0484_0081, 0x6080_a080_801c_044a,
    0x2088_4008_1100_8020, 0x000c_40aa_0420_8070, 0x4100_8004_4090_0220,
    0x0000_0000_4811_2050, 0x8182_00d0_6201_2a10, 0x0402_0084_0450_8302,
    0x0000_1000_2010_1002, 0x0020_0404_2050_4912, 0x0002_0040_0811_8814,
    0x1000_8106_5008_4024, 0x1002_a030_0240_8804, 0x2104_2948_0118_1420,
    0x0841_0802_4050_0812, 0x4406_0090_0000_4884, 0x0080_0820_0401_2412,
    0x0080_0908_8080_8183, 0x0300_1200_2040_0410, 0x021a_0901_0082_2002,
];

const ROOK_MAGICS: [u64; Square::COUNT] = [
    0x0140_0004_0080_9300, 0x1320_0009_0200_0240, 0x0080_0191_0c00_8180,
    0x0040_0200_0440_1040, 0x0040_0100_00d0_1120, 0x0080_0480_2008_4050,
    0x0040_0040_0008_0228, 0x0040_0440_000a_2a0a, 0x0040_0031_0101_0102,
    0x80c4_2000_1210_8100, 0x4010_c002_0400_0c01, 0x0220_4001_0325_0002,
    0x0002_6002_0000_4001, 0x0040_2000_5240_0020, 0x0c00_1000_2002_0008,
    0x9080_2010_0020_0004, 0x2200_2010_0008_0004, 0x8080_4c00_2020_0191,
    0x0045_3830_0000_9100, 0x0030_0028_0002_0040, 0x0040_1040_0098_8084,
    0x0108_0010_0080_0415, 0x0014_0050_0040_0009, 0x0d21_0010_01c0_0045,
    0x00c0_0030_0020_0024, 0x0040_0030_0028_0004, 0x0040_0210_0009_1102,
    0x2008_a204_0800_0d00, 0x2000_1000_8401_0040, 0x0144_0800_0800_8001,
    0x5010_2400_1000_26a2, 0x1040_0200_0800_1010, 0x1200_2000_2800_5010,
    0x4280_0300_3002_0898, 0x0480_0814_1001_1004, 0x0340_0004_0800_110a,
    0x0010_1000_010c_0021, 0x0009_2108_0008_0082, 0x0610_0002_0004_00a7,
    0xa224_0800_9008_00c0, 0x9220_0820_0100_0801, 0x1040_0080_0114_0030,
    0x0040_0022_2004_0008, 0x0280_0012_4008_010c, 0x0040_0084_0494_0002,
    0x0040_0408_0001_0200, 0x0090_0008_0900_2100, 0x2800_0800_0100_0201,
    0x1400_0200_0100_0201, 0x0180_0810_1401_8004, 0x1100_0080_0040_0201,
    0x0080_0040_0020_0201, 0x0420_8000_1000_0201, 0x2841_c000_8020_0209,
    0x0120_0024_0104_0001, 0x0145_1000_0101_000b, 0x0040_0800_0080_8001,
    0x0834_0001_8804_8001, 0x4001_2100_0080_0205, 0x0488_9a80_0740_0201,
    0x2080_0440_8020_0062, 0x0080_0040_0286_1002, 0x0000_c008_4204_9024,
    0x8040_0002_0202_0011, 0x0040_0404_002c_0100, 0x2080_0282_0200_0102,
    0x8100_0408_0059_0224, 0x2040_0090_0480_0010, 0x0040_0450_0040_0408,
    0x2200_2400_2080_2008, 0x4080_0420_0220_0204, 0x0040_00b0_000a_00a2,
    0x000a_6000_0081_0100, 0x0014_1000_0d00_1180, 0x0002_2001_0100_1080,
    0x1000_2001_4104_e120, 0x2407_2001_0000_4810, 0x8014_4000_a084_5050,
    0x1000_2000_6003_0c18, 0x4004_2000_2001_0102, 0x0140_6000_2101_0302,
];