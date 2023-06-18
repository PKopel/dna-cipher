use std::{
    fmt::Debug,
    ops::{Add, BitXor, Sub},
};

pub mod xors;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DNA {
    A,
    C,
    G,
    T,
}

impl Debug for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DNA::A => "'A'",
            DNA::C => "'C'",
            DNA::G => "'G'",
            DNA::T => "'T'",
        };
        write!(f, "{}", s)
    }
}

impl BitXor for DNA {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        xors::dnaxor_1(self, rhs)
    }
}

impl Sub for DNA {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::A => match rhs {
                DNA::A => DNA::A,
                DNA::C => DNA::C,
                DNA::G => DNA::T,
                DNA::T => DNA::G,
            },
            Self::C => match rhs {
                DNA::A => DNA::C,
                DNA::C => DNA::A,
                DNA::G => DNA::G,
                DNA::T => DNA::T,
            },
            Self::G => match rhs {
                DNA::A => DNA::G,
                DNA::C => DNA::T,
                DNA::G => DNA::A,
                DNA::T => DNA::C,
            },
            Self::T => match rhs {
                DNA::A => DNA::T,
                DNA::C => DNA::G,
                DNA::G => DNA::C,
                DNA::T => DNA::A,
            },
        }
    }
}

impl Add for DNA {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::A => match rhs {
                DNA::A => DNA::T,
                DNA::C => DNA::A,
                DNA::G => DNA::C,
                DNA::T => DNA::G,
            },
            Self::C => rhs,
            Self::G => match rhs {
                DNA::A => DNA::C,
                DNA::C => DNA::T,
                DNA::G => DNA::G,
                DNA::T => DNA::A,
            },
            Self::T => match rhs {
                DNA::A => DNA::G,
                DNA::C => DNA::T,
                DNA::G => DNA::A,
                DNA::T => DNA::C,
            },
        }
    }
}

pub struct DNAWord(pub [DNA; 4]);

impl BitXor for DNAWord {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut result = [DNA::A; 4];
        for i in 0..4 {
            result[i] = self.0[i] ^ rhs.0[i];
        }
        DNAWord(result)
    }
}

#[allow(non_snake_case)]
pub fn binary_to_DNA(b: &u8) -> [DNA; 4] {
    let mut x = *b;
    let mut result = [DNA::A; 4];
    for base in &mut result {
        // check first two bits
        *base = match (x & 0b11000000) >> 6 {
            0b00 => DNA::A,
            0b01 => DNA::G,
            0b10 => DNA::C,
            0b11 => DNA::T,
            _ => break,
        };
        x <<= 2;
    }
    result
}

#[allow(non_snake_case)]
pub fn DNA_to_binary(dna: &[DNA; 4]) -> u8 {
    let mut result = 0;
    for b in dna.iter() {
        result <<= 2;
        result |= match b {
            DNA::A => 0b00,
            DNA::G => 0b01,
            DNA::C => 0b10,
            DNA::T => 0b11,
        };
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xor() {
        assert_eq!(DNA::A ^ DNA::A, DNA::A);
        assert_eq!((DNA::A ^ DNA::G) ^ DNA::G, DNA::A);
    }

    #[test]
    fn test_sub() {
        assert_eq!(DNA::A - DNA::A, DNA::A);
        assert_eq!((DNA::A - DNA::G) - DNA::T, DNA::A);
    }

    #[test]
    fn test_add() {
        assert_eq!(DNA::A + DNA::A, DNA::T);
        assert_eq!((DNA::A + DNA::G) + DNA::G, DNA::G);
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_binary_to_DNA() {
        assert_eq!(
            binary_to_DNA(&0b00011011_u8).to_vec(),
            vec![DNA::A, DNA::G, DNA::C, DNA::T]
        );
        assert_eq!(
            binary_to_DNA(&0b00000000_u8).to_vec(),
            vec![DNA::A, DNA::A, DNA::A, DNA::A]
        );
        assert_eq!(
            binary_to_DNA(&0b01010101_u8).to_vec(),
            vec![DNA::G, DNA::G, DNA::G, DNA::G]
        );
        assert_eq!(
            binary_to_DNA(&0b10101010_u8).to_vec(),
            vec![DNA::C, DNA::C, DNA::C, DNA::C]
        );
        assert_eq!(
            binary_to_DNA(&0b11111111_u8).to_vec(),
            vec![DNA::T, DNA::T, DNA::T, DNA::T]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DNA_to_binary() {
        assert_eq!(
            DNA_to_binary(&[DNA::A, DNA::G, DNA::C, DNA::T]),
            0b00011011_u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::A, DNA::A, DNA::A, DNA::A]),
            0b00000000_u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::G, DNA::G, DNA::G, DNA::G]),
            0b01010101_u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::C, DNA::C, DNA::C, DNA::C]),
            0b10101010_u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::T, DNA::T, DNA::T, DNA::T]),
            0b11111111_u8
        );
    }
}
