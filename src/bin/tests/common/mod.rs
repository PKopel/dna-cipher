use dnac::dna::{DNA, *};

const INPUT_SIZE_BYTES: usize = 16;
pub struct Bits {
    bits: [u8; INPUT_SIZE_BYTES],
    n_bit: usize,
}

impl Bits {
    pub fn new(bits: [u8; INPUT_SIZE_BYTES]) -> Self {
        Bits {
            bits: bits,
            n_bit: 0,
        }
    }
}

impl Iterator for Bits {
    type Item = [u8; INPUT_SIZE_BYTES];

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_bit < INPUT_SIZE_BYTES * 8 {
            let u8_idx = self.n_bit / 8;
            let mask = 0b10000000 >> (self.n_bit % 8);

            let mut new_bits = self.bits.clone();
            new_bits[u8_idx] ^= mask;

            self.n_bit += 1;

            Some(new_bits)
        } else {
            None
        }
    }
}

pub fn encrypt(bits: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
    let key = include_bytes!("data/key_128B.blb")
        .iter()
        .flat_map(binary_to_DNA)
        .collect();
    let result = bits.iter().flat_map(binary_to_DNA).collect::<Vec<DNA>>();
    dnac::encrypt(result, key)
        .chunks_exact(4)
        .map(|chunk| DNA_to_binary(chunk.try_into().unwrap()))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

pub fn xor_array(a: [u8; INPUT_SIZE_BYTES], b: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
    a.into_iter()
        .zip(b)
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap()
}

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
