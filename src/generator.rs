use crate::magics;
use crate::bits;
use rand::random;

struct MagicEntry {
    mask: u128,
    magic: u128,
    shift: u8,
}

impl MagicEntry {
    fn index(&self, blockers: u128) -> usize {
        let blockers = blockers & self.mask;
        let hash = blockers.wrapping_mul(self.magic);
        let index = if self.shift < 128 { (hash >> self.shift) as usize } else { 0 };
        index
    }
}

pub fn find_and_print_all_magics(deltas: &[i8], slider_name: &str) {
    println!(
        "pub const {}_MAGICS: &[MagicEntry; Square::COUNT] = &[",
        slider_name
    );
    let mut total_table_size = 0;
    for sq in 0..81 {
        let index_bits = bits::count(magics::attack_mask(deltas, sq));
        let (entry, table) = find_magic(deltas, sq, index_bits);
        println!(
            "    MagicEntry {{ mask: 0x{:016X}, magic: 0x{:016X}, shift: {}, offset: {} }},",
            entry.mask, entry.magic, entry.shift, total_table_size
        );
        total_table_size += table.len();
    }
    println!("];");
    println!(
        "pub const {}_TABLE_SIZE: usize = {};",
        slider_name, total_table_size
    );
}

fn find_magic(deltas: &[i8], sq: u8, index_bits: u8) -> (MagicEntry, Vec<u128>) {

    let mask = magics::attack_mask(deltas, sq);
    let shift = 128 - index_bits;
    loop {
        let magic = rand_u128() & rand_u128() & rand_u128();
        let magic_entry = MagicEntry { mask, magic, shift };
        if let Some(table) = test_magic(deltas, sq, &magic_entry) {
            return (magic_entry, table);
        }
    }

}

fn test_magic(deltas: &[i8], sq: u8, magic_entry: &MagicEntry) -> Option<Vec<u128>>{

    let index_bits = 128 - magic_entry.shift;
    let table_size = 1 << index_bits;
    let mut table = vec![bits::NONE; table_size];
    let mut blockers = bits::NONE;
    loop {
        let moves = magics::sliding_attacks(deltas, sq, blockers);
        let index = magic_entry.index(blockers);
        if table[index] != bits::NONE {
            return None;
        }
        table[index] = moves;
        blockers = (blockers.wrapping_sub(magic_entry.mask)) & magic_entry.mask;
        if bits::empty(blockers) {
            break;
        }
    }

    Some(table)

}

fn rand_u128() -> u128 {
    random::<u128>()
}
