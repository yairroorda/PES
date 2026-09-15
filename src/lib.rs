#![allow(warnings)]

use std::fmt;

pub enum Key {
    Pes128,
    Pes256,
}

pub struct PES_128 {
    pub key: [u8; 16],
}

impl fmt::Display for PES_128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_grid(&self.key, f, 4)
    }
}

#[derive(PartialEq, Eq)]
pub struct Block {
    pub bytes: [u8; 16],
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_grid(&self.bytes, f, 4)
    }
}

fn format_grid(bytes: &[u8], f: &mut fmt::Formatter<'_>, cols: usize) -> fmt::Result {
    for (i, row) in bytes.chunks(cols).enumerate() {
        if i > 0 {
            writeln!(f)?;
        }

        for (j, byte) in row.iter().enumerate() {
            if j > 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x}", byte)?;
        }
    }
    Ok(())
}

fn encrypt(key: Key) {
    todo!()
}

fn encrypt_block(key: &Key, block: Block) -> Block {
    todo!()
}

pub fn subbytes(block: Block) -> Block {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_after_subbytes() {
        #[rustfmt::skip]
        let test_key: PES_128 = PES_128 {
            key: [
                0x2b, 0x7e, 0x15, 0x16,
                0x28, 0xae, 0xd2, 0xa6,
                0xab, 0xf7, 0x15, 0x88,
                0x09, 0xcf, 0x4f, 0x3c,
            ],
        };

        #[rustfmt::skip]
        let test_input: Block = Block {
            bytes: [
                0x32, 0x43, 0xf6, 0xa8,
                0x88, 0x5a, 0x30, 0x8d,
                0x31, 0x31, 0x98, 0xa2,
                0xe0, 0x37, 0x07, 0x34,
            ],
        };

        #[rustfmt::skip]
        let after_subbytes: Block = Block{
            bytes: [
                0xd4, 0xe0, 0xb8, 0x1e,
                0x27, 0xbf, 0xb4, 0x41,
                0x11, 0x98, 0x5d, 0x52,
                0xae, 0xf1, 0xe5, 0x30,
            ],
        };

        assert!(subbytes(test_input) == after_subbytes);
    }
}
