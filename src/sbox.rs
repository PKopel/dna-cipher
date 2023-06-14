use crate::dna::DNA;
use std::ops::Index;

#[derive(Copy, Clone, Debug)]
pub struct SBox {}

impl SBox {
    pub fn new() -> SBox {
        SBox {}
    }
}

impl Index<&[DNA; 4]> for SBox {
    type Output = [DNA; 4];

    fn index(&self, index: &[DNA; 4]) -> &Self::Output {
        match index[0] {
            DNA::A => match index[1] {
                DNA::A => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::A, DNA::T],
                        DNA::C => &[DNA::G, DNA::T, DNA::G, DNA::T],
                        DNA::G => &[DNA::G, DNA::T, DNA::T, DNA::A],
                        DNA::T => &[DNA::G, DNA::T, DNA::C, DNA::T],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::A, DNA::T, DNA::A, DNA::A],
                        DNA::C => &[DNA::G, DNA::C, DNA::G, DNA::T],
                        DNA::G => &[DNA::A, DNA::A, DNA::A, DNA::G],
                        DNA::T => &[DNA::A, DNA::C, DNA::C, DNA::T],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::T, DNA::T, DNA::A, DNA::C],
                        DNA::C => &[DNA::G, DNA::C, DNA::T, DNA::T],
                        DNA::G => &[DNA::G, DNA::C, DNA::C, DNA::T],
                        DNA::T => &[DNA::T, DNA::A, DNA::G, DNA::G],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::T, DNA::T, DNA::T, DNA::C],
                        DNA::C => &[DNA::C, DNA::C, DNA::C, DNA::T],
                        DNA::G => &[DNA::T, DNA::G, DNA::G, DNA::T],
                        DNA::T => &[DNA::G, DNA::T, DNA::G, DNA::C],
                    },
                },
                DNA::C => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::C, DNA::T, DNA::G, DNA::T],
                        DNA::C => &[DNA::C, DNA::G, DNA::A, DNA::T],
                        DNA::G => &[DNA::T, DNA::T, DNA::T, DNA::G],
                        DNA::T => &[DNA::A, DNA::C, DNA::G, DNA::C],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::A, DNA::T, DNA::G, DNA::A],
                        DNA::C => &[DNA::T, DNA::C, DNA::G, DNA::G],
                        DNA::G => &[DNA::C, DNA::C, DNA::G, DNA::G],
                        DNA::T => &[DNA::T, DNA::T, DNA::A, DNA::G],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::A, DNA::T, DNA::G, DNA::C],
                        DNA::C => &[DNA::T, DNA::T, DNA::G, DNA::T],
                        DNA::G => &[DNA::A, DNA::T, DNA::T, DNA::T],
                        DNA::T => &[DNA::T, DNA::A, DNA::T, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::G, DNA::T, DNA::A, DNA::G],
                        DNA::C => &[DNA::A, DNA::T, DNA::A, DNA::G],
                        DNA::G => &[DNA::T, DNA::G, DNA::C, DNA::A],
                        DNA::T => &[DNA::A, DNA::G, DNA::G, DNA::G],
                    },
                },
                DNA::G => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::T, DNA::A, DNA::C, DNA::C],
                        DNA::C => &[DNA::T, DNA::A, DNA::C, DNA::G],
                        DNA::G => &[DNA::C, DNA::A, DNA::A, DNA::C],
                        DNA::T => &[DNA::G, DNA::T, DNA::T, DNA::G],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::C, DNA::C, DNA::T, DNA::G],
                        DNA::C => &[DNA::C, DNA::C, DNA::A, DNA::C],
                        DNA::G => &[DNA::T, DNA::G, DNA::G, DNA::A],
                        DNA::T => &[DNA::C, DNA::C, DNA::T, DNA::T],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::T, DNA::T, DNA::C, DNA::C],
                        DNA::C => &[DNA::G, DNA::A, DNA::G, DNA::T],
                        DNA::G => &[DNA::G, DNA::G, DNA::C, DNA::G],
                        DNA::T => &[DNA::T, DNA::T, DNA::A, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::C, DNA::G, DNA::T, DNA::A],
                        DNA::C => &[DNA::G, DNA::T, DNA::A, DNA::C],
                        DNA::G => &[DNA::C, DNA::C, DNA::G, DNA::A],
                        DNA::T => &[DNA::T, DNA::A, DNA::A, DNA::A],
                    },
                },
                DNA::T => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::A, DNA::A, DNA::G, DNA::A],
                        DNA::C => &[DNA::A, DNA::C, DNA::A, DNA::T],
                        DNA::G => &[DNA::T, DNA::A, DNA::G, DNA::T],
                        DNA::T => &[DNA::T, DNA::A, DNA::A, DNA::T],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::A, DNA::A, DNA::G, DNA::T],
                        DNA::C => &[DNA::C, DNA::A, DNA::A, DNA::A],
                        DNA::G => &[DNA::A, DNA::G, DNA::A, DNA::C],
                        DNA::T => &[DNA::T, DNA::C, DNA::A, DNA::C],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::A, DNA::G, DNA::C, DNA::A],
                        DNA::C => &[DNA::A, DNA::A, DNA::G, DNA::G],
                        DNA::G => &[DNA::C, DNA::G, DNA::G, DNA::C],
                        DNA::T => &[DNA::C, DNA::G, DNA::C, DNA::C],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::T, DNA::C, DNA::C, DNA::T],
                        DNA::C => &[DNA::C, DNA::T, DNA::A, DNA::C],
                        DNA::G => &[DNA::A, DNA::C, DNA::G, DNA::T],
                        DNA::T => &[DNA::G, DNA::T, DNA::G, DNA::G],
                    },
                },
            },
            DNA::C => match index[1] {
                DNA::A => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::T, DNA::A, DNA::T, DNA::G],
                        DNA::C => &[DNA::A, DNA::G, DNA::A, DNA::T],
                        DNA::G => &[DNA::A, DNA::A, DNA::T, DNA::A],
                        DNA::T => &[DNA::T, DNA::C, DNA::T, DNA::A],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::T, DNA::A, DNA::G, DNA::A],
                        DNA::C => &[DNA::G, DNA::T, DNA::T, DNA::C],
                        DNA::G => &[DNA::C, DNA::C, DNA::G, DNA::T],
                        DNA::T => &[DNA::A, DNA::T, DNA::T, DNA::G],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::G, DNA::G, DNA::T, DNA::T],
                        DNA::C => &[DNA::G, DNA::A, DNA::G, DNA::A],
                        DNA::G => &[DNA::C, DNA::G, DNA::G, DNA::T],
                        DNA::T => &[DNA::A, DNA::G, DNA::G, DNA::T],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::G, DNA::A],
                        DNA::C => &[DNA::A, DNA::G, DNA::C, DNA::G],
                        DNA::G => &[DNA::G, DNA::G, DNA::T, DNA::G],
                        DNA::T => &[DNA::G, DNA::T, DNA::A, DNA::T],
                    },
                },
                DNA::C => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::T, DNA::C, DNA::A, DNA::A],
                        DNA::C => &[DNA::A, DNA::T, DNA::C, DNA::C],
                        DNA::G => &[DNA::A, DNA::T, DNA::A, DNA::C],
                        DNA::T => &[DNA::A, DNA::A, DNA::C, DNA::C],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::T, DNA::A, DNA::A, DNA::C],
                        DNA::C => &[DNA::C, DNA::C, DNA::T, DNA::A],
                        DNA::G => &[DNA::T, DNA::G, DNA::A, DNA::T],
                        DNA::T => &[DNA::G, DNA::C, DNA::A, DNA::C],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::C, DNA::G],
                        DNA::C => &[DNA::A, DNA::C, DNA::G, DNA::A],
                        DNA::G => &[DNA::A, DNA::A, DNA::G, DNA::C],
                        DNA::T => &[DNA::G, DNA::G, DNA::T, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::C, DNA::G, DNA::A, DNA::G],
                        DNA::C => &[DNA::T, DNA::C, DNA::G, DNA::A],
                        DNA::G => &[DNA::C, DNA::G, DNA::G, DNA::G],
                        DNA::T => &[DNA::G, DNA::T, DNA::C, DNA::G],
                    },
                },
                DNA::G => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::A, DNA::A],
                        DNA::C => &[DNA::G, DNA::A, DNA::T, DNA::T],
                        DNA::G => &[DNA::C, DNA::A, DNA::A, DNA::G],
                        DNA::T => &[DNA::T, DNA::G, DNA::T, DNA::A],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::G, DNA::C],
                        DNA::C => &[DNA::C, DNA::T, DNA::C, DNA::A],
                        DNA::G => &[DNA::T, DNA::C, DNA::T, DNA::C],
                        DNA::T => &[DNA::A, DNA::G, DNA::G, DNA::A],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::A, DNA::C, DNA::A, DNA::C],
                        DNA::C => &[DNA::C, DNA::G, DNA::A, DNA::A],
                        DNA::G => &[DNA::A, DNA::C, DNA::C, DNA::C],
                        DNA::T => &[DNA::C, DNA::A, DNA::C, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::T, DNA::G, DNA::T, DNA::C],
                        DNA::C => &[DNA::A, DNA::A, DNA::C, DNA::T],
                        DNA::G => &[DNA::G, DNA::G, DNA::T, DNA::C],
                        DNA::T => &[DNA::T, DNA::G, DNA::C, DNA::T],
                    },
                },
                DNA::T => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::T, DNA::C, DNA::G, DNA::T],
                        DNA::C => &[DNA::A, DNA::T, DNA::G, DNA::T],
                        DNA::G => &[DNA::T, DNA::A, DNA::C, DNA::A],
                        DNA::T => &[DNA::G, DNA::C, DNA::T, DNA::G],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::T, DNA::A],
                        DNA::C => &[DNA::T, DNA::T, DNA::G, DNA::A],
                        DNA::G => &[DNA::G, DNA::G, DNA::G, DNA::C],
                        DNA::T => &[DNA::T, DNA::C, DNA::C, DNA::C],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::C, DNA::A, DNA::T, DNA::G],
                        DNA::C => &[DNA::G, DNA::A, DNA::T, DNA::C],
                        DNA::G => &[DNA::T, DNA::G, DNA::G, DNA::G],
                        DNA::T => &[DNA::C, DNA::C, DNA::C, DNA::G],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::G, DNA::G],
                        DNA::C => &[DNA::C, DNA::C, DNA::T, DNA::C],
                        DNA::G => &[DNA::G, DNA::T, DNA::C, DNA::C],
                        DNA::T => &[DNA::A, DNA::A, DNA::C, DNA::A],
                    },
                },
            },
            DNA::G => match index[1] {
                DNA::A => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::A, DNA::A, DNA::C, DNA::G],
                        DNA::C => &[DNA::A, DNA::C, DNA::T, DNA::A],
                        DNA::G => &[DNA::C, DNA::A, DNA::A, DNA::T],
                        DNA::T => &[DNA::A, DNA::G, DNA::C, DNA::C],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::G, DNA::A, DNA::C],
                        DNA::C => &[DNA::T, DNA::G, DNA::G, DNA::C],
                        DNA::G => &[DNA::A, DNA::T, DNA::C, DNA::T],
                        DNA::T => &[DNA::C, DNA::T, DNA::A, DNA::T],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::A, DNA::G, DNA::C, DNA::T],
                        DNA::C => &[DNA::G, DNA::G, DNA::C, DNA::C],
                        DNA::G => &[DNA::G, DNA::C, DNA::T, DNA::C],
                        DNA::T => &[DNA::C, DNA::C, DNA::A, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::A, DNA::C, DNA::C, DNA::G],
                        DNA::C => &[DNA::A, DNA::C, DNA::T, DNA::T],
                        DNA::G => &[DNA::T, DNA::C, DNA::A, DNA::T],
                        DNA::T => &[DNA::C, DNA::A, DNA::G, DNA::A],
                    },
                },
                DNA::C => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::T, DNA::G, DNA::A, DNA::A],
                        DNA::C => &[DNA::C, DNA::C, DNA::C, DNA::C],
                        DNA::G => &[DNA::T, DNA::C, DNA::T, DNA::T],
                        DNA::T => &[DNA::T, DNA::T, DNA::C, DNA::T],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::G, DNA::G],
                        DNA::C => &[DNA::A, DNA::A, DNA::A, DNA::C],
                        DNA::G => &[DNA::T, DNA::T, DNA::C, DNA::G],
                        DNA::T => &[DNA::G, DNA::T, DNA::T, DNA::T],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::A, DNA::T],
                        DNA::C => &[DNA::A, DNA::T, DNA::A, DNA::T],
                        DNA::G => &[DNA::G, DNA::A, DNA::T, DNA::G],
                        DNA::T => &[DNA::C, DNA::A, DNA::G, DNA::G],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::G, DNA::G, DNA::A, DNA::A],
                        DNA::C => &[DNA::C, DNA::G, DNA::T, DNA::T],
                        DNA::G => &[DNA::A, DNA::T, DNA::T, DNA::A],
                        DNA::T => &[DNA::C, DNA::C, DNA::C, DNA::A],
                    },
                },
                DNA::G => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::G, DNA::G, DNA::A, DNA::T],
                        DNA::C => &[DNA::A, DNA::A, DNA::A, DNA::A],
                        DNA::G => &[DNA::T, DNA::G, DNA::A, DNA::G],
                        DNA::T => &[DNA::T, DNA::C, DNA::T, DNA::G],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::C, DNA::C],
                        DNA::C => &[DNA::C, DNA::T, DNA::T, DNA::C],
                        DNA::G => &[DNA::T, DNA::A, DNA::C, DNA::T],
                        DNA::T => &[DNA::A, DNA::T, DNA::C, DNA::G],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::A, DNA::C, DNA::A, DNA::A],
                        DNA::C => &[DNA::C, DNA::T, DNA::A, DNA::G],
                        DNA::G => &[DNA::T, DNA::T, DNA::T, DNA::A],
                        DNA::T => &[DNA::G, DNA::G, DNA::C, DNA::T],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::C, DNA::C],
                        DNA::C => &[DNA::G, DNA::G, DNA::C, DNA::A],
                        DNA::G => &[DNA::G, DNA::A, DNA::T, DNA::A],
                        DNA::T => &[DNA::T, DNA::A, DNA::T, DNA::T],
                    },
                },
                DNA::T => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::G, DNA::G, DNA::A, DNA::G],
                        DNA::C => &[DNA::G, DNA::A, DNA::A, DNA::A],
                        DNA::G => &[DNA::C, DNA::C, DNA::A, DNA::T],
                        DNA::T => &[DNA::C, DNA::A, DNA::T, DNA::T],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::C, DNA::T, DNA::T, DNA::A],
                        DNA::C => &[DNA::T, DNA::G, DNA::C, DNA::C],
                        DNA::G => &[DNA::C, DNA::T, DNA::G, DNA::C],
                        DNA::T => &[DNA::A, DNA::C, DNA::A, DNA::G],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::C, DNA::G, DNA::A, DNA::C],
                        DNA::C => &[DNA::A, DNA::T, DNA::C, DNA::A],
                        DNA::G => &[DNA::C, DNA::G, DNA::T, DNA::G],
                        DNA::T => &[DNA::T, DNA::T, DNA::G, DNA::G],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::A, DNA::G, DNA::A, DNA::A],
                        DNA::C => &[DNA::T, DNA::T, DNA::A, DNA::T],
                        DNA::G => &[DNA::T, DNA::T, DNA::T, DNA::T],
                        DNA::T => &[DNA::T, DNA::G, DNA::A, DNA::C],
                    },
                },
            },
            DNA::T => match index[1] {
                DNA::A => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::C, DNA::T, DNA::C, DNA::C],
                        DNA::C => &[DNA::A, DNA::C, DNA::G, DNA::G],
                        DNA::G => &[DNA::G, DNA::T, DNA::C, DNA::A],
                        DNA::T => &[DNA::A, DNA::C, DNA::T, DNA::C],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::T, DNA::C, DNA::C, DNA::A],
                        DNA::C => &[DNA::G, DNA::T, DNA::G, DNA::A],
                        DNA::G => &[DNA::T, DNA::G, DNA::T, DNA::G],
                        DNA::T => &[DNA::A, DNA::G, DNA::T, DNA::T],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::A, DNA::G, DNA::T, DNA::A],
                        DNA::C => &[DNA::C, DNA::T, DNA::G, DNA::A],
                        DNA::G => &[DNA::C, DNA::C, DNA::G, DNA::C],
                        DNA::T => &[DNA::T, DNA::A, DNA::G, DNA::C],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::C, DNA::T],
                        DNA::C => &[DNA::C, DNA::A, DNA::C, DNA::T],
                        DNA::G => &[DNA::C, DNA::T, DNA::T, DNA::G],
                        DNA::T => &[DNA::C, DNA::A, DNA::C, DNA::C],
                    },
                },
                DNA::C => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::T, DNA::C, DNA::A, DNA::G],
                        DNA::C => &[DNA::C, DNA::G, DNA::C, DNA::A],
                        DNA::G => &[DNA::T, DNA::T, DNA::C, DNA::A],
                        DNA::T => &[DNA::A, DNA::G, DNA::A, DNA::G],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::C, DNA::G, DNA::C, DNA::T],
                        DNA::C => &[DNA::C, DNA::A, DNA::G, DNA::T],
                        DNA::G => &[DNA::A, DNA::G, DNA::T, DNA::C],
                        DNA::T => &[DNA::T, DNA::C, DNA::C, DNA::G],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::C, DNA::G],
                        DNA::C => &[DNA::C, DNA::A, DNA::T, DNA::C],
                        DNA::G => &[DNA::T, DNA::G, DNA::C, DNA::G],
                        DNA::T => &[DNA::C, DNA::G, DNA::G, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::T, DNA::A, DNA::T, DNA::C],
                        DNA::C => &[DNA::A, DNA::C, DNA::C, DNA::A],
                        DNA::G => &[DNA::G, DNA::G, DNA::G, DNA::G],
                        DNA::T => &[DNA::T, DNA::G, DNA::T, DNA::T],
                    },
                },
                DNA::G => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::G, DNA::T, DNA::A, DNA::A],
                        DNA::C => &[DNA::C, DNA::T, DNA::G, DNA::G],
                        DNA::G => &[DNA::A, DNA::T, DNA::T, DNA::C],
                        DNA::T => &[DNA::G, DNA::C, DNA::G, DNA::C],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::C, DNA::A, DNA::G],
                        DNA::C => &[DNA::G, DNA::G, DNA::G, DNA::T],
                        DNA::G => &[DNA::A, DNA::T, DNA::G, DNA::G],
                        DNA::T => &[DNA::C, DNA::T, DNA::C, DNA::G],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::C, DNA::A],
                        DNA::C => &[DNA::T, DNA::T, DNA::G, DNA::C],
                        DNA::G => &[DNA::A, DNA::A, DNA::A, DNA::T],
                        DNA::T => &[DNA::A, DNA::A, DNA::T, DNA::C],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::C, DNA::A, DNA::G, DNA::C],
                        DNA::C => &[DNA::A, DNA::G, DNA::T, DNA::G],
                        DNA::G => &[DNA::T, DNA::A, DNA::A, DNA::G],
                        DNA::T => &[DNA::C, DNA::G, DNA::T, DNA::C],
                    },
                },
                DNA::T => match index[2] {
                    DNA::A => match index[3] {
                        DNA::A => &[DNA::C, DNA::A, DNA::T, DNA::A],
                        DNA::C => &[DNA::C, DNA::A, DNA::C, DNA::G],
                        DNA::G => &[DNA::C, DNA::C, DNA::A, DNA::G],
                        DNA::T => &[DNA::A, DNA::A, DNA::T, DNA::G],
                    },
                    DNA::C => match index[3] {
                        DNA::A => &[DNA::G, DNA::A, DNA::A, DNA::G],
                        DNA::C => &[DNA::A, DNA::C, DNA::T, DNA::G],
                        DNA::G => &[DNA::C, DNA::G, DNA::C, DNA::G],
                        DNA::T => &[DNA::A, DNA::A, DNA::T, DNA::T],
                    },
                    DNA::G => match index[3] {
                        DNA::A => &[DNA::C, DNA::T, DNA::T, DNA::T],
                        DNA::C => &[DNA::G, DNA::A, DNA::A, DNA::C],
                        DNA::G => &[DNA::T, DNA::C, DNA::G, DNA::C],
                        DNA::T => &[DNA::G, DNA::C, DNA::C, DNA::A],
                    },
                    DNA::T => match index[3] {
                        DNA::A => &[DNA::C, DNA::T, DNA::A, DNA::A],
                        DNA::C => &[DNA::C, DNA::T, DNA::C, DNA::T],
                        DNA::G => &[DNA::G, DNA::G, DNA::G, DNA::A],
                        DNA::T => &[DNA::A, DNA::G, DNA::G, DNA::C],
                    },
                },
            },
        }
    }
}
