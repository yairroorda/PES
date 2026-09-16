#![allow(warnings)]

use PES::{Block, PES_128, add_round_key, shiftrows, subbytes};

fn main() {
    #[rustfmt::skip]
    let test_key: PES_128 = PES_128 {
        key: [
            0x2b, 0x28, 0xab, 0x09,
            0x7e, 0xae, 0xf7, 0xcf,
            0x15, 0xd2, 0x15, 0x4f,
            0x16, 0xa6, 0x88, 0x3c,
        ],
    };

    #[rustfmt::skip]
    let test_input: Block = Block {
        items: [
            0x32, 0x88, 0x31, 0xe0,
            0x43, 0x5a, 0x31, 0x37,
            0xf6, 0x30, 0x98, 0x07,
            0xa8, 0x8d, 0xa2, 0x34,
        ],
    };
    println!("---Input---");
    println!("{test_input}");

    let after_add_round_key = add_round_key(test_input, &test_key);
    println!("---what it is after add_round_key---");
    println!("{after_add_round_key}");

    let after_shiftrows_mine: Block = subbytes(after_add_round_key);
    println!("---what it is- after subbytes--");
    println!("{after_shiftrows_mine}");
}
