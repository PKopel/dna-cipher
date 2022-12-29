use std::ops::{Add, BitXor};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DNA {
    A,
    C,
    G,
    T,
}

impl BitXor for DNA {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        return match self {
            Self::A => rhs,
            Self::C => match rhs {
                DNA::A => DNA::C,
                DNA::C => DNA::A,
                DNA::G => DNA::T,
                DNA::T => DNA::G,
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
        };
    }
}

impl Add for DNA {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return match self {
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
        };
    }
}

#[allow(non_snake_case)]
pub fn binary_to_DNA(b: &u8) -> Vec<DNA> {
    let mut x = *b;
    let mut result = vec![];
    for _ in 0..4 {
        // check first two bits
        match (x & 0b11000000) >> 6 {
            0b00 => result.push(DNA::A),
            0b01 => result.push(DNA::G),
            0b10 => result.push(DNA::C),
            0b11 => result.push(DNA::T),
            _ => {}
        };
        x = x << 2;
    }
    return result;
}

#[allow(non_snake_case)]
pub fn DNA_to_binary(dna: &[DNA; 4]) -> u8 {
    let mut result = 0;
    for b in dna.into_iter() {
        result = result << 2;
        result |= match b {
            DNA::A => 0b00,
            DNA::G => 0b01,
            DNA::C => 0b10,
            DNA::T => 0b11,
        };
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_xor() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!((a ^ b) ^ b, a);
            }
        }
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
            binary_to_DNA(&(0b00011011 as u8)),
            vec![DNA::A, DNA::G, DNA::C, DNA::T]
        );
        assert_eq!(
            binary_to_DNA(&(0b00000000 as u8)),
            vec![DNA::A, DNA::A, DNA::A, DNA::A]
        );
        assert_eq!(
            binary_to_DNA(&(0b01010101 as u8)),
            vec![DNA::G, DNA::G, DNA::G, DNA::G]
        );
        assert_eq!(
            binary_to_DNA(&(0b10101010 as u8)),
            vec![DNA::C, DNA::C, DNA::C, DNA::C]
        );
        assert_eq!(
            binary_to_DNA(&(0b11111111 as u8)),
            vec![DNA::T, DNA::T, DNA::T, DNA::T]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_DNA_to_binary() {
        assert_eq!(
            DNA_to_binary(&[DNA::A, DNA::G, DNA::C, DNA::T]),
            0b00011011 as u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::A, DNA::A, DNA::A, DNA::A]),
            0b00000000 as u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::G, DNA::G, DNA::G, DNA::G]),
            0b01010101 as u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::C, DNA::C, DNA::C, DNA::C]),
            0b10101010 as u8
        );
        assert_eq!(
            DNA_to_binary(&[DNA::T, DNA::T, DNA::T, DNA::T]),
            0b11111111 as u8
        );
    }
}
