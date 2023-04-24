use log::trace;

pub mod dna;
use dna::{
    xors::{get_xor, word_xor},
    DNAWord, DNA,
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
const INTRON_SIZE: usize = 6;
// intron size of 6 with target size of 18 uses 3 out of 4 pairs in key

pub struct DNAC {
    sbox: SBox,
    key: Vec<DNA>,
}

impl DNAC {
    pub fn new(key: Vec<DNA>) -> DNAC {
        DNAC {
            sbox: SBox::new(),
            key: key,
        }
    }

    fn expand_key(&self, key: &[DNA; KEY_SIZE]) -> [DNA; TARGET_SIZE] {
        let (fst, snd) = key.split_at(4);
        let mut fst = fst.try_into().unwrap();
        let mut snd = snd.try_into().unwrap();
        let mut expanded = [DNA::A; TARGET_SIZE];

        expanded[..KEY_SIZE].copy_from_slice(key);
        let mut expanded_len = KEY_SIZE;

        while expanded_len < TARGET_SIZE {
            fst = word_xor(fst, self.sbox[&snd]);
            snd = word_xor(snd, fst);
            expanded[expanded_len..expanded_len + 4].copy_from_slice(&fst);
            expanded[expanded_len + 4..expanded_len + 8].copy_from_slice(&snd);
            expanded_len += 8;
        }

        expanded
    }

    fn round(&self, input: &[DNA; INPUT_SIZE], key: &[DNA; KEY_SIZE]) -> [DNA; INPUT_SIZE] {
        let mut result = [DNA::A; INPUT_SIZE];
        result.copy_from_slice(input.as_slice());
        let (source, target) = result.split_at_mut(SOURCE_SIZE);
        let (intron_patterns, xor_selector) = key.split_at(KEY_SIZE - 2);

        let mut intron = self.expand_key(key);
        let mut intron_len = 0;

        let mut intron_idx = 0;
        let mut source_idx = 0;
        while source_idx < SOURCE_SIZE - 1 && intron_len < TARGET_SIZE - 2 {
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

        // transform introns with sbox
        let intron: Vec<DNA> = intron
            .chunks_exact(4)
            .flat_map(|chunk| self.sbox[chunk.try_into().unwrap()].into_iter())
            .collect();

        // use last two bases of key to select the xor definition
        let dna_xor = get_xor(xor_selector);
        for i in 0..TARGET_SIZE {
            // order is important - target must be the first argument
            target[i] = dna_xor(target[i], intron[i]);
        }
        result
    }

    pub fn encrypt(self, input: Vec<DNA>) -> Vec<DNA> {
        let ciphertext = input
            .chunks(INPUT_SIZE)
            .flat_map(|chunk| {
                let mut input_chunk = [DNA::A; INPUT_SIZE];
                let mut key_chunks = self.key.chunks_exact(KEY_SIZE).peekable();

                // in case last chunk is shorter than INPUT_SIZE bases the rest will be filled with A's
                input_chunk[0..chunk.len()].copy_from_slice(chunk);

                while let Some(key_chunk) = key_chunks.next() {
                    // try_into changes slices to arrays of fixed length
                    let result = self.round(&input_chunk, key_chunk.try_into().unwrap());
                    if key_chunks.peek().is_some() {
                        // swap head with tail as per the Feistel algorithm
                        input_chunk[0..TARGET_SIZE]
                            .copy_from_slice(&result[SOURCE_SIZE..INPUT_SIZE]);
                        input_chunk[TARGET_SIZE..INPUT_SIZE]
                            .copy_from_slice(&result[0..SOURCE_SIZE]);
                    } else {
                        // for last round we need to keep the order to be able to decrypt the message
                        input_chunk.copy_from_slice(&result);
                    }
                }
                input_chunk.to_vec()
            })
            .collect::<Vec<DNA>>();
        ciphertext
    }

    pub fn decrypt(self, input: Vec<DNA>) -> Result<Vec<DNA>, String> {
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
                let mut key_chunks = self.key.chunks_exact(KEY_SIZE).rev().peekable();

                // each chunk will be of length INPUT_SIZE
                input_chunk.copy_from_slice(chunk);

                while let Some(key_chunk) = key_chunks.next() {
                    // try_into changes slices to arrays of fixed length
                    let result = self.round(&input_chunk, key_chunk.try_into().unwrap());
                    if key_chunks.peek().is_some() {
                        // swap head with tail as per the Feistel algorithm
                        input_chunk[0..SOURCE_SIZE]
                            .copy_from_slice(&result[TARGET_SIZE..INPUT_SIZE]);
                        input_chunk[SOURCE_SIZE..INPUT_SIZE]
                            .copy_from_slice(&result[0..TARGET_SIZE]);
                    } else {
                        // for last round we need to keep the order to be able to decrypt the message
                        input_chunk.copy_from_slice(&result);
                    }
                }
                input_chunk.to_vec()
            })
            .collect::<Vec<DNA>>();
        Ok(plaintext)
    }
}
