use std::sync::Arc;

use log::trace;

pub mod bits;
pub mod dna;
use dna::{
    binary_to_DNA,
    xors::{get_xor, word_xor},
    DNA,
};

mod sbox;
use sbox::SBox;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}

const TARGET_SIZE: usize = 20;
const SOURCE_SIZE: usize = 44;
const INPUT_SIZE: usize = 64;
const KEY_SIZE: usize = 8;
const INTRON_SIZE: usize = 8;
// intron size of 6 with target size of 18 uses 3 out of 4 pairs in key

pub struct DNAC {
    sbox: SBox,
    key: Arc<[[DNA; KEY_SIZE]]>,
}

impl DNAC {
    pub fn new_default(key: Vec<DNA>) -> DNAC {
        DNAC::new(key, 46) // nuber of rounds based on test results
    }

    pub fn new(key: Vec<DNA>, rounds: usize) -> DNAC {
        let sbox = SBox::new();
        let key = DNAC::expand_key(key, sbox, rounds);
        DNAC { sbox, key }
    }

    fn expand_key(key: Vec<DNA>, sbox: SBox, rounds: usize) -> Arc<[[DNA; KEY_SIZE]]> {
        let original = key
            .chunks_exact(4)
            .map(|chunk| chunk.try_into().unwrap())
            .collect::<Vec<[DNA; 4]>>();
        let n = original.len() / 2; // number of 8-base chunks
        if n >= rounds {
            return key
                .chunks_exact(KEY_SIZE)
                .map(|chunk| chunk.try_into().unwrap())
                .take(rounds)
                .collect();
        }
        let rcs = vec![0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36];
        let mut rcs = rcs.iter().map(binary_to_DNA).cycle();

        let expanded_final_size = (rounds / 2 + 1) * 4; // expanded contains 4-base chunks, rounds use 8-base chunks
        let mut expanded = vec![[DNA::A; 4]; expanded_final_size];
        let mut expanded_len = original.len();
        expanded[0..expanded_len].copy_from_slice(&original[..]);

        let input_key_words = n / 2; // number of 16-base words in the original key
        while expanded_len < expanded_final_size {
            match (expanded_len / 4) % input_key_words {
                0 => {
                    let rci = rcs.next().unwrap();
                    for i in 0..4 {
                        expanded[expanded_len + i] = word_xor(
                            word_xor(
                                expanded[expanded_len - n * 2 + i],
                                sbox[&expanded[expanded_len - 4 + (i + 1) % 4]],
                            ),
                            rci,
                        )
                    }
                }

                4 if n > 24 => {
                    for i in 0..4 {
                        expanded[expanded_len + i] = word_xor(
                            expanded[expanded_len - n * 2 + i],
                            sbox[&expanded[expanded_len - 4 + i]],
                        )
                    }
                }

                _ => {
                    for i in 0..4 {
                        expanded[expanded_len + i] = word_xor(
                            expanded[expanded_len - n * 2 + i],
                            expanded[expanded_len - 4 + i],
                        )
                    }
                }
            }
            expanded_len += 4;
        }

        expanded
            .iter()
            .flat_map(|w| w.to_vec())
            .collect::<Vec<DNA>>()
            .chunks_exact(KEY_SIZE)
            .map(|chunk| chunk.try_into().unwrap())
            .take(rounds)
            .collect()
    }

    fn round(&self, input: &[DNA; INPUT_SIZE], key: &[DNA; KEY_SIZE]) -> [DNA; INPUT_SIZE] {
        let mut result = [DNA::A; INPUT_SIZE];
        result.copy_from_slice(input.as_slice());
        let (source, target) = result.split_at_mut(SOURCE_SIZE);
        let (intron_patterns, xor_selector) = key.split_at(KEY_SIZE - 2);

        let mut intron = [DNA::A; TARGET_SIZE];
        let mut intron_len = 0;

        let mut intron_idx = 0;
        let mut source_idx = 0;
        while source_idx < SOURCE_SIZE - 1 && intron_idx < 6 {
            if intron_patterns[intron_idx..intron_idx + 2] == source[source_idx..source_idx + 2] {
                let cp_len = min!(
                    SOURCE_SIZE - 1 - source_idx, // limit to the end of the source block
                    TARGET_SIZE - intron_len,     // limit to the size of target block
                    INTRON_SIZE
                );
                intron[intron_len..intron_len + cp_len]
                    .copy_from_slice(&source[source_idx..source_idx + cp_len]);
                intron_len += cp_len;
                source_idx += cp_len;
                intron_idx += 2;
            } else {
                source_idx += 1;
            }
        }
        trace!("intron_len = {}", intron_len);

        // use last two bases of key to select the xor definition
        let dna_xor = get_xor(xor_selector);

        intron
            // transform introns with sbox
            .chunks_exact(4)
            .flat_map(|chunk| self.sbox[chunk.try_into().unwrap()].into_iter())
            .enumerate()
            // order is important - target must be the first argument
            .for_each(|(i, intron_base)| target[i] = dna_xor(target[i], intron_base));

        // return result table with both source and target blocks
        result
    }

    pub fn encrypt(&self, input: Vec<DNA>) -> Vec<DNA> {
        let ciphertext = input
            .chunks(INPUT_SIZE)
            .flat_map(|chunk| {
                let mut input_chunk = [DNA::A; INPUT_SIZE];
                let mut key_chunks = self.key.iter().peekable();

                // in case last chunk is shorter than INPUT_SIZE bases the rest will be filled with A's
                input_chunk[0..chunk.len()].copy_from_slice(chunk);

                while let Some(key_chunk) = key_chunks.next() {
                    // try_into changes slices to arrays of fixed length
                    let result = self.round(&input_chunk, key_chunk);
                    if key_chunks.peek().is_some() {
                        // swap head with tail as per the Feistel algorithm
                        input_chunk[0..TARGET_SIZE]
                            .copy_from_slice(&result[SOURCE_SIZE..INPUT_SIZE]);
                        input_chunk[TARGET_SIZE..INPUT_SIZE]
                            .copy_from_slice(&result[0..SOURCE_SIZE]);
                    } else {
                        // for last round we need to keep the order to be able to decrypt the message
                        input_chunk = result;
                    }
                }
                input_chunk.to_vec()
            })
            .collect::<Vec<DNA>>();
        ciphertext
    }

    pub fn decrypt(&self, input: Vec<DNA>) -> Result<Vec<DNA>, String> {
        if input.len() % INPUT_SIZE != 0 {
            return Err(format!(
                "illegal input, length should be a multiple of {}",
                INPUT_SIZE
            ));
        }
        let plaintext = input
            .chunks_exact(INPUT_SIZE)
            .flat_map(|chunk| {
                let mut input_chunk = [DNA::A; INPUT_SIZE];
                let mut key_chunks = self.key.iter().rev().peekable();

                // each chunk will be of length INPUT_SIZE
                input_chunk.copy_from_slice(chunk);

                while let Some(key_chunk) = key_chunks.next() {
                    // try_into changes slices to arrays of fixed length
                    let result = self.round(&input_chunk, key_chunk);
                    if key_chunks.peek().is_some() {
                        // swap head with tail as per the Feistel algorithm
                        input_chunk[0..SOURCE_SIZE]
                            .copy_from_slice(&result[TARGET_SIZE..INPUT_SIZE]);
                        input_chunk[SOURCE_SIZE..INPUT_SIZE]
                            .copy_from_slice(&result[0..TARGET_SIZE]);
                    } else {
                        // for last round we need to keep the order to be able to decrypt the message
                        input_chunk = result;
                    }
                }
                input_chunk.to_vec()
            })
            .collect::<Vec<DNA>>();
        Ok(plaintext)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_encrypt_decrypt() {
        let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
        let random_msg = rand::thread_rng().gen::<[u8; 16]>();
        let key = random_bytes
            .iter()
            .flat_map(binary_to_DNA)
            .collect::<Vec<DNA>>();
        let msg = random_msg
            .iter()
            .flat_map(binary_to_DNA)
            .collect::<Vec<DNA>>();
        let cipher = DNAC::new_default(key);
        assert_eq!(msg, cipher.decrypt(cipher.encrypt(msg.clone())).unwrap());
    }
}
