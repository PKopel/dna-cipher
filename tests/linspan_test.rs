use rand::{seq::SliceRandom, thread_rng};

mod common;
use common::{encrypt, xor_array, Bits, INPUT_SIZE_BYTES};

fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element)
                .collect()
        })
        .collect()
}

#[test]
fn linear_span_test() {
    let zeroes = [0u8; 16];
    // each will have only one '1', so they are linearly independent
    let mut bits = Bits::new(zeroes).collect::<Vec<[u8; INPUT_SIZE_BYTES]>>();
    // limit the number of used bits
    bits.shuffle(&mut thread_rng());
    bits = bits
        .chunks(16)
        .map(|chunk| chunk.iter().fold(zeroes, |acc, &x| xor_array(acc, x)))
        .collect();
    let bits = &bits[0..7];
    let inputs: Vec<[u8; INPUT_SIZE_BYTES]> = powerset(bits)
        .iter()
        .map(|chunk| chunk.iter().fold(zeroes, |acc, &&x| xor_array(acc, x)))
        .collect();
    let linspan_matrix = inputs
        .iter()
        .map(|&x| encrypt(x))
        .collect::<Vec<[u8; INPUT_SIZE_BYTES]>>();
}
