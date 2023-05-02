// use std::collections::HashMap;

use dnac::dna::{DNA, *};

pub const INPUT_SIZE_BYTES: usize = 16;
pub struct BitsOne {
    bits: [u8; INPUT_SIZE_BYTES],
    n_bit: usize,
}

impl BitsOne {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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

#[allow(dead_code)]
pub fn encrypt(bits: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
    let key = include_bytes!("data/key_32B.blb")
        .iter()
        .flat_map(binary_to_DNA)
        .collect();
    let result = bits.iter().flat_map(binary_to_DNA).collect::<Vec<DNA>>();
    let cipher = dnac::DNAC::new(key);
    cipher
        .encrypt(result)
        .chunks_exact(4)
        .map(|chunk| DNA_to_binary(chunk.try_into().unwrap()))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

#[allow(dead_code)]
pub fn xor_array(a: [u8; INPUT_SIZE_BYTES], b: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

#[allow(dead_code)]
pub fn check_ones(a: &u8) -> [u32; 8] {
    let mut mask = 0b1000_0000;
    let mut result = [0; 8];
    for r in &mut result {
        if a & mask > 0 {
            *r = 1;
        }
        mask >>= 1;
    }
    result
}

#[allow(dead_code)]
pub fn powerset<T>(s: &[T]) -> impl Iterator<Item = Vec<&T>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter()
            .enumerate()
            .filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element)
            .collect()
    })
}
