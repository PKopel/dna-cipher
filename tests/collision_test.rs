use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;
use kdam::{tqdm, TqdmIterator};
use rand::Rng;
use rayon::{
    self,
    prelude::{ParallelBridge, ParallelIterator},
};

use crate::common::{encrypt, BitsAll};

mod common;

const N: u32 = 12;
const PROBABILITIES: [f64; 5] = [0.206246, 0.194005, 0.219834, 0.183968, 0.195947];

fn x2_test(bins: [u32; 5]) -> f64 {
    let mut e = 0.0;
    let size = 4096 * 128; // 4294967296u64;
    bins.iter()
        .enumerate()
        .map(|(i, l)| {
            e = PROBABILITIES[i] * (size as f64);
            ((*l as f64) - e).powf(2.0) / e
        })
        .sum()
}

#[test]
fn collision_test() {
    //let mut bins = [0; 5];
    let data = include_bytes!("common/data/texts_16MB.blb");
    // let key = GenericArray::from([0u8; 16]);
    // let cipher = Aes128::new(&key);
    let bins: [u32; 5] = data
        .chunks_exact(16)
        .take(128)
        .tqdm()
        .par_bridge()
        .map(|input| {
            let mut bins = [0; 5];
            let mut outputs = HashSet::<[u8; 16]>::new();
            let mut collisions = 0;
            let input_bits = input.try_into().unwrap();
            let bits = BitsAll::new(input_bits, N);
            // let mut fst_block = GenericArray::from(input_bits);
            // cipher.encrypt_block(&mut fst_block);
            for bits in bits {
                // let mut block = GenericArray::from(bits);
                // cipher.encrypt_block(&mut block);
                let output = encrypt(bits);
                // let xored = xor_array(
                //     fst_block.as_slice().try_into().unwrap(),
                //     block.as_slice().try_into().unwrap(),
                // );
                if outputs.contains(&output) {
                    collisions += 1;
                } else {
                    outputs.insert(output);
                }
            }
            match collisions {
                _ if collisions < 117 => bins[0] += 1,
                _ if collisions < 123 => bins[1] += 1,
                _ if collisions < 129 => bins[2] += 1,
                _ if collisions < 135 => bins[3] += 1,
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
    // println!("{:?}", sac_matrix);
    fs::write("collisions.txt", format!("{:?}", bins)).expect("Error saving collision numbers");
    let val = x2_test(bins);
    // assert!(val < 11.345) // based on lookup table value for 4 degrees
}
