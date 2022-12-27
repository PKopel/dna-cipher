use crate::dna::DNA;
use std::cmp::min;

pub fn round(input: Vec<DNA>, mut key: Vec<DNA>) -> (Vec<DNA>, Vec<DNA>) {
    let mut intron = vec![];
    let mut intron_len = 0;
    let mut tail_len = input.len();
    let mut fst = key.pop().unwrap();
    let mut snd = key.pop().unwrap();
    let mut i = 0;
    // assuming even lengths
    while intron_len != tail_len {
        if input[i] == fst && input[i + 1] == snd {
            let cp_len = min((tail_len - intron_len) / 2, 8);
            intron.append(&mut Vec::from_iter(input[i..i + cp_len].iter().cloned()));
            intron_len += cp_len;
            tail_len -= cp_len;
            i += cp_len;
            fst = key.pop().unwrap();
            snd = key.pop().unwrap();
        } else {
            i += 1;
            tail_len -= 1;
        }
    }

    let tail = &input[i..];
    for j in 0..intron_len {
        intron[j] = intron[j] ^ tail[j];
    }
    return (input[0..i].to_vec(), intron);
}
