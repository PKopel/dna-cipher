// use std::collections::HashMap;

use dnac::{
    bits::INPUT_SIZE_BYTES,
    dna::{DNA, *},
};

pub struct Test {
    cipher: dnac::DNAC,
}

impl Test {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let key = include_bytes!("data/key_32B.blb")
            .iter()
            .flat_map(binary_to_DNA)
            .collect();
        let cipher = dnac::DNAC::new(key);
        Test { cipher }
    }

    #[allow(dead_code)]
    pub fn encrypt(&self, bits: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
        let result = bits.iter().flat_map(binary_to_DNA).collect::<Vec<DNA>>();
        self.cipher
            .encrypt(result)
            .chunks_exact(4)
            .map(|chunk| DNA_to_binary(chunk.try_into().unwrap()))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }
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
