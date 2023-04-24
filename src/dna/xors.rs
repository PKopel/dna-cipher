use log::trace;

use super::DNA;

// Only this is the 'full' xor, others are not comutative
pub fn dnaxor_1(a: DNA, b: DNA) -> DNA {
    match a {
        DNA::A => b,
        DNA::C => match b {
            DNA::A => DNA::C,
            DNA::C => DNA::A,
            DNA::G => DNA::T,
            DNA::T => DNA::G,
        },
        DNA::G => match b {
            DNA::A => DNA::G,
            DNA::C => DNA::T,
            DNA::G => DNA::A,
            DNA::T => DNA::C,
        },
        DNA::T => match b {
            DNA::A => DNA::T,
            DNA::C => DNA::G,
            DNA::G => DNA::C,
            DNA::T => DNA::A,
        },
    }
}

pub fn dnaxor_2(a: DNA, b: DNA) -> DNA {
    match a {
        DNA::A => match b {
            DNA::A => DNA::A,
            DNA::C => DNA::C,
            DNA::G => DNA::T,
            DNA::T => DNA::G,
        },
        DNA::C => match b {
            DNA::A => DNA::C,
            DNA::C => DNA::A,
            DNA::G => DNA::G,
            DNA::T => DNA::T,
        },
        DNA::G => match b {
            DNA::A => DNA::G,
            DNA::C => DNA::T,
            DNA::G => DNA::C,
            DNA::T => DNA::A,
        },
        DNA::T => match b {
            DNA::A => DNA::T,
            DNA::C => DNA::G,
            DNA::G => DNA::A,
            DNA::T => DNA::C,
        },
    }
}

pub fn dnaxor_3(a: DNA, b: DNA) -> DNA {
    match a {
        DNA::A => match b {
            DNA::A => DNA::A,
            DNA::C => DNA::G,
            DNA::G => DNA::C,
            DNA::T => DNA::T,
        },
        DNA::C => match b {
            DNA::A => DNA::C,
            DNA::C => DNA::T,
            DNA::G => DNA::A,
            DNA::T => DNA::G,
        },
        DNA::G => match b {
            DNA::A => DNA::G,
            DNA::C => DNA::A,
            DNA::G => DNA::T,
            DNA::T => DNA::C,
        },
        DNA::T => match b {
            DNA::A => DNA::T,
            DNA::C => DNA::C,
            DNA::G => DNA::G,
            DNA::T => DNA::A,
        },
    }
}

pub fn dnaxor_4(a: DNA, b: DNA) -> DNA {
    match a {
        DNA::A => match b {
            DNA::A => DNA::A,
            DNA::C => DNA::G,
            DNA::G => DNA::T,
            DNA::T => DNA::C,
        },
        DNA::C => match b {
            DNA::A => DNA::C,
            DNA::C => DNA::T,
            DNA::G => DNA::G,
            DNA::T => DNA::A,
        },
        DNA::G => match b {
            DNA::A => DNA::G,
            DNA::C => DNA::A,
            DNA::G => DNA::C,
            DNA::T => DNA::T,
        },
        DNA::T => match b {
            DNA::A => DNA::T,
            DNA::C => DNA::C,
            DNA::G => DNA::A,
            DNA::T => DNA::G,
        },
    }
}

pub fn dnaxor_5(a: DNA, b: DNA) -> DNA {
    match a {
        DNA::A => match b {
            DNA::A => DNA::A,
            DNA::C => DNA::T,
            DNA::G => DNA::C,
            DNA::T => DNA::G,
        },
        DNA::C => match b {
            DNA::A => DNA::C,
            DNA::C => DNA::G,
            DNA::G => DNA::A,
            DNA::T => DNA::T,
        },
        DNA::G => match b {
            DNA::A => DNA::G,
            DNA::C => DNA::C,
            DNA::G => DNA::T,
            DNA::T => DNA::A,
        },
        DNA::T => match b {
            DNA::A => DNA::T,
            DNA::C => DNA::A,
            DNA::G => DNA::G,
            DNA::T => DNA::C,
        },
    }
}

pub fn dnaxor_6(a: DNA, b: DNA) -> DNA {
    match a {
        DNA::A => match b {
            DNA::A => DNA::A,
            DNA::C => DNA::T,
            DNA::G => DNA::G,
            DNA::T => DNA::C,
        },
        DNA::C => match b {
            DNA::A => DNA::C,
            DNA::C => DNA::G,
            DNA::G => DNA::T,
            DNA::T => DNA::A,
        },
        DNA::G => match b {
            DNA::A => DNA::G,
            DNA::C => DNA::C,
            DNA::G => DNA::A,
            DNA::T => DNA::T,
        },
        DNA::T => match b {
            DNA::A => DNA::T,
            DNA::C => DNA::A,
            DNA::G => DNA::C,
            DNA::T => DNA::G,
        },
    }
}

pub fn get_xor(key: &[DNA]) -> fn(DNA, DNA) -> DNA {
    trace!("get xor key = {:?}", key);
    match key {
        [DNA::A, DNA::A] | [DNA::C, DNA::C] | [DNA::G, DNA::G] | [DNA::T, DNA::T] => dnaxor_1,
        [DNA::A, DNA::C] | [DNA::A, DNA::G] | [DNA::A, DNA::T] => dnaxor_2,
        [DNA::C, DNA::A] | [DNA::C, DNA::G] | [DNA::C, DNA::T] => dnaxor_3,
        [DNA::G, DNA::A] | [DNA::G, DNA::C] | [DNA::G, DNA::T] => dnaxor_4,
        [DNA::T, DNA::C] | [DNA::T, DNA::G] | [DNA::T, DNA::A] => dnaxor_5,
        _ => dnaxor_6, // should never match
    }
}

pub fn word_xor(a: [DNA; 4], b: [DNA; 4]) -> [DNA; 4] {
    let mut result = [DNA::A; 4];
    for i in 0..4 {
        result[i] = a[i] ^ b[i];
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dnaxor_1() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!(dnaxor_1(dnaxor_1(a, b), b), a);
            }
        }
    }

    #[test]
    fn test_dnaxor_2() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!(dnaxor_2(dnaxor_2(a, b), b), a);
            }
        }
    }

    #[test]
    fn test_dnaxor_3() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!(dnaxor_3(dnaxor_3(a, b), b), a);
            }
        }
    }

    #[test]
    fn test_dnaxor_4() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!(dnaxor_4(dnaxor_4(a, b), b), a);
            }
        }
    }

    #[test]
    fn test_dnaxor_5() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!(dnaxor_5(dnaxor_5(a, b), b), a);
            }
        }
    }

    #[test]
    fn test_dnaxor_6() {
        for a in [DNA::A, DNA::C, DNA::G, DNA::T] {
            for b in [DNA::A, DNA::C, DNA::G, DNA::T] {
                assert_eq!(dnaxor_6(dnaxor_6(a, b), b), a);
            }
        }
    }
}
