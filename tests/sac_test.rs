use common::{check_ones, encrypt, xor_array};
use kdam::tqdm;
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod common;

fn sac_probabilities(x: usize) -> f64 {
    (match x {
        0 => 0.200224,
        1 => 0.199937,
        2 => 0.199677,
        3 => 0.199937,
        _ => 0.200224,
    }) * 16384f64 // 128 * 128
}

fn x2_test(matrix: [[u32; 128]; 128]) -> f64 {
    let mut bins = vec![vec![]; 5];
    for x in matrix.iter().flat_map(|row| row.iter()) {
        match x {
            _ if *x < 523857 => bins[0].push(x),
            _ if *x < 524158 => bins[1].push(x),
            _ if *x < 524417 => bins[2].push(x),
            _ if *x < 524718 => bins[3].push(x),
            _ => bins[4].push(x),
        }
    }
    let mut e = 0.0;
    bins.iter()
        .map(|v| v.len())
        .enumerate()
        .map(|(i, l)| {
            e = sac_probabilities(i);
            ((l as f64) - e).powf(2.0) / e
        })
        .sum()
}

#[test]
fn sac_test() {
    let mut sac_matrix = [[0; 128]; 128];
    let data = include_bytes!("common/tests.blb");
    for input in tqdm!(data.chunks_exact(16)) {
        let mut bits = common::Bits::new(input.try_into().unwrap());
        let fst_output = encrypt(bits.next().unwrap());
        bits.zip(&mut sac_matrix)
            .par_bridge()
            .for_each(|(bits, results)| {
                let output = encrypt(bits);
                let xored = xor_array(fst_output, output);
                for (i, v) in xored.iter().flat_map(check_ones).enumerate() {
                    results[i] += v;
                }
            });
    }
    let val = x2_test(sac_matrix);
    println!("{}", val)
}
