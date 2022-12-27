use crate::dna::DNA;

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (::std::cmp::min($x, min!($($z),*)));
}

const TARGET_SIZE: usize = 16;
const SOURCE_SIZE: usize = 48;
const INPUT_SIZE: usize = 64;
const KEY_SIZE: usize = 8;

pub fn round(
    input: &[DNA; INPUT_SIZE],
    key: &[DNA; KEY_SIZE],
) -> ([DNA; SOURCE_SIZE], [DNA; TARGET_SIZE]) {
    let mut intron = vec![];
    let mut intron_len = 0;

    let mut source = [DNA::A; SOURCE_SIZE];
    let mut target = [DNA::A; TARGET_SIZE];
    source.clone_from_slice(&input[0..SOURCE_SIZE]);
    target.clone_from_slice(&input[SOURCE_SIZE..INPUT_SIZE]);

    let mut source_idx = 0;
    let mut key_idx = 0;
    while source_idx < SOURCE_SIZE - 1 && intron_len < TARGET_SIZE {
        if source[source_idx] == key[key_idx] && source[source_idx + 1] == key[key_idx + 1] {
            let cp_len = min!(SOURCE_SIZE - 1 - source_idx, TARGET_SIZE - intron_len, 8);
            intron.append(&mut Vec::from_iter(
                source[source_idx..source_idx + cp_len].iter().cloned(),
            ));
            intron_len += cp_len;
            source_idx += cp_len;
            key_idx += 2;
        } else {
            source_idx += 1;
        }
    }
    for i in 0..intron_len {
        target[i] = intron[i] ^ target[i];
    }
    return (source, target);
}

pub fn encrypt(input: Vec<DNA>, key: Vec<DNA>) -> Vec<DNA> {
    let ciphertext = input
        .chunks(INPUT_SIZE)
        .flat_map(|chunk| {
            let mut input_chunk = [DNA::A; INPUT_SIZE];
            let mut key_chunks = key.chunks_exact(KEY_SIZE).peekable();

            // in case last chunk is shorter than INPUT_SIZE bases the rest will be filled with A's
            input_chunk[0..chunk.len()].clone_from_slice(chunk);

            while let Some(key_chunk) = key_chunks.next() {
                // try_into changes slices to arrays of fixed length
                let (h, t) = round(&input_chunk, key_chunk.try_into().unwrap());
                if key_chunks.peek().is_some() {
                    // swap head with tail as per the Feistel algorithm
                    input_chunk[0..TARGET_SIZE].clone_from_slice(&t);
                    input_chunk[TARGET_SIZE..INPUT_SIZE].clone_from_slice(&h);
                } else {
                    // for last round we need to keep the order to be able to decrypt the message
                    input_chunk[0..SOURCE_SIZE].clone_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_SIZE].clone_from_slice(&t);
                }
            }
            return input_chunk.to_vec();
        })
        .collect::<Vec<DNA>>();
    return ciphertext;
}

pub fn decrypt(input: Vec<DNA>, key: Vec<DNA>) -> Result<Vec<DNA>, String> {
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
            let mut key_chunks = key.chunks_exact(KEY_SIZE).rev().peekable();

            // each chunk will be of length INPUT_SIZE
            input_chunk.clone_from_slice(chunk);

            while let Some(key_chunk) = key_chunks.next() {
                // try_into changes slices to arrays of fixed length
                let (mut h, mut t) = round(&input_chunk, key_chunk.try_into().unwrap());
                if key_chunks.peek().is_some() {
                    // swap head with tail as per the Feistel algorithm
                    input_chunk[0..SOURCE_SIZE].clone_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_SIZE].clone_from_slice(&t);
                    t.clone_from_slice(&input_chunk[0..TARGET_SIZE]);
                    h.clone_from_slice(&input_chunk[TARGET_SIZE..INPUT_SIZE]);
                    input_chunk[0..SOURCE_SIZE].clone_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_SIZE].clone_from_slice(&t);
                } else {
                    // for last round we need to keep the order as it is
                    input_chunk[0..SOURCE_SIZE].clone_from_slice(&h);
                    input_chunk[SOURCE_SIZE..INPUT_SIZE].clone_from_slice(&t);
                }
            }
            return input_chunk.to_vec();
        })
        .collect::<Vec<DNA>>();
    return Ok(plaintext);
}