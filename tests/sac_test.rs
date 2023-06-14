use std::fs;

use common::{check_ones, xor_array};
use kdam::tqdm;
use rayon::prelude::{ParallelBridge, ParallelIterator};

// use aes::cipher::{generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit};
// use aes::Aes128;

mod common;

const PROBABILITIES: [f64; 5] = [0.200224, 0.199937, 0.199677, 0.199937, 0.200224];

fn x2_test(matrix: [[u32; 128]; 128]) -> f64 {
    let mut bins = [0; 5];
    for x in matrix.iter().flat_map(|row| row.iter()) {
        match x {
            _ if *x < 523857 => bins[0] += 1,
            _ if *x < 524158 => bins[1] += 1,
            _ if *x < 524417 => bins[2] += 1,
            _ if *x < 524718 => bins[3] += 1,
            _ => bins[4] += 1,
        }
    }
    let mut e = 0.0;
    bins.iter()
        .enumerate()
        .map(|(i, l)| {
            e = PROBABILITIES[i] * 16384.0;
            ((*l as f64) - e).powf(2.0) / e
        })
        .sum()
}

#[test]
fn sac_test() {
    let test = common::Test::new();
    let mut sac_matrix = [[0; 128]; 128];
    let data = include_bytes!("common/data/texts_16MB.blb");
    // let key = GenericArray::from([0u8; 16]);
    // let cipher = Aes128::new(&key);
    for input in tqdm!(data.chunks_exact(16)) {
        let input_bits = input.try_into().unwrap();
        let bits = common::BitsOne::new(input_bits);
        // let mut fst_block = GenericArray::from(input_bits);
        // cipher.encrypt_block(&mut fst_block);
        let fst_output = test.encrypt(input.try_into().unwrap());
        bits.zip(&mut sac_matrix)
            .par_bridge()
            .for_each(|(bits, results)| {
                // let mut block = GenericArray::from(bits);
                // cipher.encrypt_block(&mut block);
                let output = test.encrypt(bits);
                // let xored = xor_array(
                //     fst_block.as_slice().try_into().unwrap(),
                //     block.as_slice().try_into().unwrap(),
                // );
                let xored = xor_array(fst_output, output);
                for (i, v) in xored.iter().flat_map(check_ones).enumerate() {
                    results[i] += v;
                }
            });
    }
    // println!("{:?}", sac_matrix);
    fs::write("sac_matrix.txt", format!("{:?}", sac_matrix)).expect("Error saving SAC matrix");
    let val = x2_test(sac_matrix);
    assert!(val < 13.277) // based on lookup table value for 4 degrees
}
