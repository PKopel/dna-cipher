use std::collections::HashSet;
use std::fs;

use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit},
    Aes128,
};
use kdam::TqdmIterator;
use rayon::{
    self,
    prelude::{ParallelBridge, ParallelIterator},
};

use crate::common::{encrypt, BitsAll};

mod common;

const N: u32 = 12;
const T: usize = 2; // 12 bits < 2 u8
const SIZE: usize = 2_usize.pow(20); // 4096 * 128; // 4294967296u64;
const PROBABILITIES: [f64; 5] = [0.199176, 0.204681, 0.197862, 0.203232, 0.195049];

fn x2_test(bins: [u32; 5]) -> f64 {
    let mut e = 0.0;
    bins.iter()
        .enumerate()
        .map(|(i, l)| {
            e = PROBABILITIES[i] * (SIZE as f64);
            ((*l as f64) - e).powf(2.0) / e
        })
        .sum()
}

#[test]
fn collision_test() {
    let data = include_bytes!("common/data/texts_16MB.blb");
    let key = GenericArray::from([0u8; 16]);
    let cipher = Aes128::new(&key);
    let bins: [u32; 5] = data
        .chunks_exact(16)
        .take(SIZE)
        .tqdm()
        .par_bridge()
        .map(|input| {
            let mut bins = [0; 5];
            let mut outputs = HashSet::<[u8; T]>::new();
            let mut coverage = 0;
            let input_bits = input.try_into().unwrap();
            let bits = BitsAll::new(input_bits, N);

            for bits in bits {
                // let encrypted = encrypt(bits);
                // let output: [u8; 2] = encrypted[..T].try_into().unwrap();
                let mut block = GenericArray::from(bits);
                cipher.encrypt_block(&mut block);
                let mut output: [u8; T] = block.as_slice()[..T].try_into().unwrap();
                output[0] &= 0b0000_1111; // clear first 4 bits to compare only 12
                if !outputs.contains(&output) {
                    coverage += 1;
                    outputs.insert(output);
                }
            }
            match coverage {
                _ if coverage < 2573 => bins[0] += 1,
                _ if coverage < 2585 => bins[1] += 1,
                _ if coverage < 2595 => bins[2] += 1,
                _ if coverage < 2607 => bins[3] += 1,
                _ => bins[4] += 1,
            }
            bins
        })
        .reduce(
            || [0; 5],
            |mut acc, x| {
                for i in 0..5 {
                    acc[i] += x[i];
                }
                acc
            },
        );

    fs::write("coverage.txt", format!("{:?}", bins)).expect("Error saving coverage numbers");
    let val = x2_test(bins);
    assert!(val < 13.277) // based on lookup table value for 4 degrees
}
