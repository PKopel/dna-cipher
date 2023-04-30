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

fn compute_rank(mut a: Vec<Vec<u8>>) -> usize {
    let n = a.len();
    let m = a[0].len();

    let mut rank = 0;
    let mut row_selected = vec![false; n];
    for i in 0..m {
        let mut j = 0;
        for k in 0..n {
            if !row_selected[k] && a[k][i] > 0 {
                j = k;
                break;
            }
        }

        if j != n {
            rank += 1;
            row_selected[j] = true;
            for p in i + 1..m {
                if a[j][i] != 0 {
                    a[j][p] = (a[j][p] / a[j][i]) % 2;
                }
            }
            for k in 0..n {
                if k != j && a[k][i] > 0 {
                    for p in i + 1..m {
                        a[k][p] = (2 + a[k][p] - (a[j][p] * a[k][i]) % 2) % 2;
                    }
                }
            }
        }
    }
    return rank;
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
        .map(|&x| {
            encrypt(x)
                .iter()
                .flat_map(|byte| {
                    let mut bits = [0; 8];
                    let mut byte = *byte;
                    for i in 0..8 {
                        bits[i] = if byte & 0b1000_0000 > 0 { 1 } else { 0 };
                        byte <<= 1;
                    }
                    bits
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let rank = compute_rank(linspan_matrix);
    println!("{}", rank);
}
