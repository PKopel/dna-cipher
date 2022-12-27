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

pub fn DNA_to_binary(dna: &[DNA; 4]) -> u8 {
    let mut result = 0;
    for b in dna.into_iter() {
        result |= match b {
            DNA::A => 0b00,
            DNA::G => 0b01,
            DNA::C => 0b10,
            DNA::T => 0b11,
        };
        result = result << 2;
    }
    return result;
}
