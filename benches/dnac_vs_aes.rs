use aes::{
    cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit},
    Aes256,
};
use dnac::{
    dna::{DNA, *},
    DNAC,
};
use kdam::tqdm;
use std::time::{Duration, Instant};
use std::{fs, io};

const INPUT_SIZE_BYTES: usize = 16;

struct Bench {
    dnac: DNAC,
    aes: Aes256,
}

impl Bench {
    fn new() -> Self {
        let key = include_bytes!("data/key_32B.blb");
        let dnac_key = key.iter().flat_map(binary_to_DNA).collect();
        let aes_key = GenericArray::from(*key);

        Bench {
            dnac: DNAC::new_default(dnac_key),
            aes: Aes256::new(&aes_key),
        }
    }

    fn encrypt_dnac(&self, bits: Vec<DNA>) -> Vec<DNA> {
        // let result = bits.iter().flat_map(binary_to_DNA).collect::<Vec<DNA>>();
        self.dnac.encrypt(bits)
        // .chunks_exact(4)
        // .map(|chunk| DNA_to_binary(chunk.try_into().unwrap()))
        // .collect::<Vec<u8>>()
        // .try_into()
        // .unwrap()
    }

    fn decrypt_dnac(&self, bits: Vec<DNA>) -> Vec<DNA> {
        // let result = bits.iter().flat_map(binary_to_DNA).collect::<Vec<DNA>>();
        self.dnac.decrypt(bits).unwrap()
        // .chunks_exact(4)
        // .map(|chunk| DNA_to_binary(chunk.try_into().unwrap()))
        // .collect::<Vec<u8>>()
        // .try_into()
        // .unwrap()
    }

    fn encrypt_aes(&self, bits: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
        let mut fst_block = GenericArray::from(bits);
        self.aes.encrypt_block(&mut fst_block);
        fst_block.as_slice().try_into().unwrap()
    }

    fn decrypt_aes(&self, bits: [u8; INPUT_SIZE_BYTES]) -> [u8; INPUT_SIZE_BYTES] {
        let mut fst_block = GenericArray::from(bits);
        self.aes.decrypt_block(&mut fst_block);
        fst_block.as_slice().try_into().unwrap()
    }
}

#[derive(Clone, Copy, Default)]
struct BenchResults {
    dnac_enc: Duration,
    dnac_dec: Duration,
    dnac_correct: bool,
    aes_enc: Duration,
    aes_dec: Duration,
    aes_correct: bool,
}

// #[cfg(aes_force_soft)]
fn main() -> io::Result<()> {
    let data = include_bytes!("data/texts_16MB.blb");
    let dnac_data = data
        .chunks_exact(16)
        .map(|bits| bits.iter().flat_map(binary_to_DNA).collect::<Vec<DNA>>())
        .collect::<Vec<Vec<DNA>>>();

    let bench = Bench::new();

    let mut results = [BenchResults {
        ..Default::default()
    }; 10];

    for i in tqdm!(0..10) {
        // dnac section
        let now = Instant::now();
        let dnac_enc: Vec<Vec<DNA>> = dnac_data
            .iter()
            .map(|input| bench.encrypt_dnac(input.clone()))
            .collect();
        results[i].dnac_enc = now.elapsed();
        let now = Instant::now();
        let dnac_dec: Vec<DNA> = dnac_enc
            .iter()
            .flat_map(|input| bench.decrypt_dnac(input.clone()))
            .collect();
        results[i].dnac_dec = now.elapsed();
        results[i].dnac_correct = dnac_data
            .iter()
            .flatten()
            .zip(dnac_dec)
            .all(|(&a, b)| a == b);

        // AES section
        let now = Instant::now();
        let aes_enc: Vec<[u8; 16]> = data
            .chunks_exact(16)
            .map(|input| bench.encrypt_aes(input.try_into().unwrap()))
            .collect();
        results[i].aes_enc = now.elapsed();
        let now = Instant::now();
        let aes_dec: Vec<u8> = aes_enc
            .iter()
            .flat_map(|chunk| bench.decrypt_aes(*chunk))
            .collect();
        results[i].aes_dec = now.elapsed();
        results[i].aes_correct = data.iter().zip(aes_dec).all(|(&a, b)| a == b);
    }

    let result_string = results
        .iter()
        .map(|r| {
            format!(
                "{},{},{},{},{},{}\n",
                r.dnac_enc.as_nanos(),
                r.dnac_dec.as_nanos(),
                r.dnac_correct,
                r.aes_enc.as_nanos(),
                r.aes_dec.as_nanos(),
                r.aes_correct
            )
        })
        .fold(
            "dnac_enc,dnac_dec,dnac_correct,aes_enc,aes_dec,aes_correct\n".to_owned(),
            |a, b| a + &b,
        );

    fs::write("dnac_vs_aes.csv", result_string).expect("Error saving bench results");

    Ok(())
}
