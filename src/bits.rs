pub const INPUT_SIZE_BYTES: usize = 16;
pub struct BitsOne {
    bits: [u8; INPUT_SIZE_BYTES],
    n_bit: usize,
}

impl BitsOne {
    pub fn new(bits: [u8; INPUT_SIZE_BYTES]) -> Self {
        BitsOne { bits, n_bit: 0 }
    }
}

impl Iterator for BitsOne {
    type Item = [u8; INPUT_SIZE_BYTES];

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_bit < INPUT_SIZE_BYTES * 8 {
            let u8_idx = self.n_bit / 8;
            let mask = 0b10000000 >> (self.n_bit % 8);

            let mut new_bits = self.bits;
            new_bits[u8_idx] ^= mask;

            self.n_bit += 1;

            Some(new_bits)
        } else {
            None
        }
    }
}

pub struct BitsAll {
    bits: [u8; INPUT_SIZE_BYTES],
    bits_iter: Vec<Vec<usize>>,
    n_bit: usize,
    n_change: u32,
}

impl BitsAll {
    pub fn new(bits: [u8; INPUT_SIZE_BYTES], n_change: u32) -> Self {
        let bits_iter = powerset(&Vec::from_iter(0..(n_change as usize)))
            .map(|v| v.iter().map(|&&a| a).collect::<Vec<usize>>())
            .collect();
        BitsAll {
            bits,
            bits_iter,
            n_bit: 0,
            n_change,
        }
    }
}

impl Iterator for BitsAll {
    type Item = [u8; INPUT_SIZE_BYTES];

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_bit < 2usize.pow(self.n_change) {
            let idxs = &self.bits_iter[self.n_bit];
            let mut new_bits = self.bits.clone();
            for idx in idxs {
                let i = idx / 8;
                let j = idx % 8;
                new_bits[i] ^= 0b1000_0000 >> j;
            }

            self.n_bit += 1;
            Some(new_bits)
        } else {
            None
        }
    }
}

pub struct BitsTwo {
    bits: [u8; INPUT_SIZE_BYTES],
    fst_bit: usize,
    snd_bit: usize,
}

impl BitsTwo {
    pub fn new(bits: [u8; INPUT_SIZE_BYTES]) -> Self {
        BitsTwo {
            bits,
            fst_bit: 0,
            snd_bit: 1,
        }
    }
}

impl Iterator for BitsTwo {
    type Item = [u8; INPUT_SIZE_BYTES];

    fn next(&mut self) -> Option<Self::Item> {
        // first bit reached the end - finish sequence
        if self.fst_bit >= INPUT_SIZE_BYTES * 8 {
            return None;
        }
        // second bit reached the end - move first one up
        // eg. 01000001 -> 00110000
        if self.snd_bit >= INPUT_SIZE_BYTES * 8 {
            self.fst_bit += 1;
            self.snd_bit = self.fst_bit + 1;
        }
        let fst_u8_idx = self.fst_bit / 8;
        let fst_mask = 0b10000000 >> (self.fst_bit % 8);

        let snd_u8_idx = self.snd_bit / 8;
        let snd_mask = 0b10000000 >> (self.snd_bit % 8);

        let mut new_bits = self.bits;
        new_bits[fst_u8_idx] ^= fst_mask;
        new_bits[snd_u8_idx] ^= snd_mask;

        self.snd_bit += 1;
        Some(new_bits)
    }
}

pub fn powerset<T>(s: &[T]) -> impl Iterator<Item = Vec<&T>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter()
            .enumerate()
            .filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element)
            .collect()
    })
}
