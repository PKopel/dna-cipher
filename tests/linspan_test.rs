use std::fs;

// use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod common;
use common::xor_array;
use dnac::bits::{powerset, BitsOne, INPUT_SIZE_BYTES};
use kdam::tqdm;

// use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
// use aes::Aes128;

fn compute_rank(mut mat: Vec<Vec<bool>>) -> usize {
    let n = mat.len();
    let m = mat[0].len();
    let mut rank = n;
    for i in 0..n {
        if !mat[i][i] {
            let mut j = i + 1;
            while j < n && !mat[j][i] {
                j += 1;
            }
            if j == n {
                rank -= 1;
                continue;
            } else {
                for k in i..m {
                    let t = mat[i][k];
                    mat[i][k] = mat[j][k];
                    mat[j][k] = t;
                }
            }
        }
        for j in i + 1..n {
            if mat[j][i] {
                for k in i..m {
                    mat[j][k] = mat[j][k] ^ mat[i][k];
                }
            }
        }
    }
    rank
}

const REPETITIONS: usize = 2097152; // 1048576;
const PROBABILITIES: [f64; 3] = [0.133636, 0.577576, 0.288788];

fn x2_test(ranks: [usize; 3]) -> f64 {
    let mut e = 0.0;
    ranks
        .iter()
        .enumerate()
        .map(|(i, l)| {
            e = PROBABILITIES[i] * (REPETITIONS as f64);
            ((*l as f64) - e).powf(2.0) / e
        })
        .sum()
}

// Not sure if this correctly described in paper, fails for both dnac and aes

#[test]
fn linear_span_test() {
    let test = common::Test::new();
    // let key = GenericArray::from([0u8; 16]);
    // let cipher = Aes128::new(&key);

    // let mut ranks = [0; REPETITIONS];
    let mut bins = [0; 3];

    let zeroes = [0u8; 16];
    // each will have only one '1', so they are linearly independent
    let base_bits = BitsOne::new(zeroes).collect::<Vec<[u8; INPUT_SIZE_BYTES]>>();
    // let mut inputs = base_bits
    //     .permutations(128)
    //     .step_by(128)
    //     .unique()
    //     .take(REPETITIONS)
    //     .collect::<Vec<Vec<[u8; 16]>>>();
    // inputs.shuffle(&mut thread_rng());
    // for bits in inputs.iter().take(128) {
    for _ in tqdm!(0..REPETITIONS) {
        let mut bits = base_bits.clone();
        bits.shuffle(&mut thread_rng());
        // limit the number of used bits
        let bits: Vec<[u8; INPUT_SIZE_BYTES]> = bits
            .chunks(16)
            .map(|chunk| chunk.iter().fold(zeroes, |acc, &x| xor_array(acc, x)))
            .collect();
        let bits = &bits[0..7];
        let inputs: Vec<[u8; INPUT_SIZE_BYTES]> = powerset(bits)
            .par_bridge()
            .map(|chunk| chunk.iter().fold(zeroes, |acc, &&x| xor_array(acc, x)))
            .collect();
        let linspan_matrix = inputs
            .iter()
            .par_bridge()
            .map(|&x| {
                // let mut block = GenericArray::from(x);
                // cipher.encrypt_block(&mut block);
                // block
                test.encrypt(x)
                    .iter()
                    .flat_map(|byte| {
                        let mut bits = [false; 8];
                        let mut byte = *byte;
                        for i in 0..8 {
                            bits[i] = byte & 0b1000_0000 > 0;
                            byte <<= 1;
                        }
                        bits
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        // ranks[r] = compute_rank(linspan_matrix);
        let rank = compute_rank(linspan_matrix);
        // println!("{}", rank);
        match rank {
            _ if rank < 127 => bins[0] += 1,
            _ if rank == 127 => bins[1] += 1,
            _ => bins[2] += 1,
        }
    }
    fs::write("linspan_ranks.txt", format!("{:?}", bins)).expect("Error saving linspan results");
    let val = x2_test(bins);
    println!("{}", val);
    assert!(val < 9.210)
}
