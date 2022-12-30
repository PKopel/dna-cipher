use crate::dna::{xors::get_xor, DNA};
use log::trace;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}

pub(crate) const KEY_SIZE: usize = 128;
pub(crate) const INPUT_CHUNK_SIZE: usize = 64;
const TARGET_SIZE: usize = 18;
const SOURCE_SIZE: usize = 46;
const KEY_CHUNK_SIZE: usize = 8;
const INTRON_SIZE: usize = 6;
// intron size of 6 with target size of 18 uses 3 out of 4 pairs in key

fn round(
    input: &[DNA; INPUT_CHUNK_SIZE],
    key: &[DNA; KEY_CHUNK_SIZE],
) -> ([DNA; SOURCE_SIZE], [DNA; TARGET_SIZE]) {
    let mut intron = [DNA::A; TARGET_SIZE];
    let mut intron_len = 0;

    let mut source = [DNA::A; SOURCE_SIZE];
    let mut target = [DNA::A; TARGET_SIZE];
    source.copy_from_slice(&input[0..SOURCE_SIZE]);
    target.copy_from_slice(&input[SOURCE_SIZE..INPUT_CHUNK_SIZE]);

    let mut source_idx = 0;
    let mut key_idx = 0;
    while source_idx < SOURCE_SIZE - 1 && intron_len < TARGET_SIZE {
        if source[source_idx] == key[key_idx] && source[source_idx + 1] == key[key_idx + 1] {
            let cp_len = min!(
                SOURCE_SIZE - 1 - source_idx, // limit to the end of the source block
                TARGET_SIZE - intron_len,     // limit to the size of target block
                INTRON_SIZE
            );
            intron[intron_len..intron_len + cp_len]
                .copy_from_slice(&source[source_idx..source_idx + cp_len]);
            intron_len += cp_len;
            source_idx += cp_len;
            key_idx += 2;
        } else {
            source_idx += 1;
        }
    }
    trace!("intron_len = {}", intron_len);
    // use last two bases of key to select the xor definition
    let dnaxor = get_xor(&key[KEY_CHUNK_SIZE - 2..KEY_CHUNK_SIZE]);
    for i in 0..TARGET_SIZE {
        // order is important - target must be the first argument
        target[i] = dnaxor(target[i], intron[i]);
    }
    return (source, target);
}

pub fn encrypt(input: Vec<DNA>, key: [DNA; KEY_SIZE]) -> Vec<DNA> {
    let ciphertext = input
        .chunks(INPUT_CHUNK_SIZE)
        .flat_map(|chunk| {
            let mut input_chunk = [DNA::A; INPUT_CHUNK_SIZE];
            let mut key_chunks = key.chunks_exact(KEY_CHUNK_SIZE).peekable();

            // in case last chunk is shorter than INPUT_SIZE bases the rest will be filled with A's
            input_chunk[0..chunk.len()].copy_from_slice(chunk);

            while let Some(key_chunk) = key_chunks.next() {
                // try_into changes slices to arrays of fixed length
                let (h, t) = round(&input_chunk, key_chunk.try_into().unwrap());
                if key_chunks.peek().is_some() {
                    // swap head with tail as per the Feistel algorithm
                    input_chunk[0..TARGET_SIZE].copy_from_slice(&t);
                    input_chunk[TARGET_SIZE..INPUT_CHUNK_SIZE].copy_from_slice(&h);
                } else {
                    // for last round we need to keep the order to be able to decrypt the message
                    input_chunk[0..SOURCE_SIZE].copy_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_CHUNK_SIZE].copy_from_slice(&t);
                }
            }
            return input_chunk.to_vec();
        })
        .collect::<Vec<DNA>>();
    return ciphertext;
}

pub fn decrypt(input: Vec<DNA>, key: [DNA; KEY_SIZE]) -> Result<Vec<DNA>, String> {
    if input.len() % INPUT_CHUNK_SIZE != 0 {
        return Err(format!(
            "illegal input, length should be a multiple of {}",
            INPUT_CHUNK_SIZE
        ));
    }
    let plaintext = input
        .chunks_exact(INPUT_CHUNK_SIZE)
        .flat_map(|chunk| {
            let mut input_chunk = [DNA::A; INPUT_CHUNK_SIZE];
            let mut key_chunks = key.chunks_exact(KEY_CHUNK_SIZE).rev().peekable();

            // each chunk will be of length INPUT_SIZE
            input_chunk.copy_from_slice(chunk);

            while let Some(key_chunk) = key_chunks.next() {
                // try_into changes slices to arrays of fixed length
                let (mut h, mut t) = round(&input_chunk, key_chunk.try_into().unwrap());
                if key_chunks.peek().is_some() {
                    // swap head with tail as per the Feistel algorithm
                    input_chunk[0..SOURCE_SIZE].copy_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_CHUNK_SIZE].copy_from_slice(&t);
                    t.copy_from_slice(&input_chunk[0..TARGET_SIZE]);
                    h.copy_from_slice(&input_chunk[TARGET_SIZE..INPUT_CHUNK_SIZE]);
                    input_chunk[0..SOURCE_SIZE].copy_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_CHUNK_SIZE].copy_from_slice(&t);
                } else {
                    // for last round we need to keep the order as it is
                    input_chunk[0..SOURCE_SIZE].copy_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_CHUNK_SIZE].copy_from_slice(&t);
                }
            }
            return input_chunk.to_vec();
        })
        .collect::<Vec<DNA>>();
    return Ok(plaintext);
}
