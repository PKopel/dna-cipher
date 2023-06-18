use clap::{Parser, Subcommand};
use dnac::dna::{self, DNA};
use kdam::tqdm;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Write},
};

const INPUT_SIZE_BYTES: usize = 16;
const RAND_FILE: &str = "/dev/urandom";
const ZERO_FILE: &str = "/dev/zero";

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq, Hash)]
enum Commands {
    KeyAv,
    PlaintextAv,
    Correlation,
    BlockChaining,
    Random,
    KeyLD,
    PlaintextLD,
    KeyHD,
    PlaintextHD,
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::KeyAv => write!(f, "key_av.blb"),
            Commands::PlaintextAv => write!(f, "plaintext_av.blb"),
            Commands::Correlation => write!(f, "correlation.blb"),
            Commands::BlockChaining => write!(f, "block_chaining.blb"),
            Commands::Random => write!(f, "random.blb"),
            Commands::KeyLD => write!(f, "key_low_density.blb"),
            Commands::PlaintextLD => write!(f, "plaintext_low_density.blb"),
            Commands::KeyHD => write!(f, "key_high_density.blb"),
            Commands::PlaintextHD => write!(f, "plaintext_high_density.blb"),
        }
    }
}

pub struct BitsOne {
    bits: [u8; INPUT_SIZE_BYTES],
    n_bit: usize,
}

impl BitsOne {
    #[allow(dead_code)]
    pub fn new(bits: [u8; INPUT_SIZE_BYTES]) -> Self {
        BitsOne { bits, n_bit: 0 }
    }
}

impl Iterator for BitsOne {
    type Item = [u8; INPUT_SIZE_BYTES];

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_bit < INPUT_SIZE_BYTES * 8 {
            let u8_idx = self.n_bit / 8;
            let mask = 0b10000000 >> (self.n_bit % 8);

            let mut new_bits = self.bits;
            new_bits[u8_idx] ^= mask;

            self.n_bit += 1;

            Some(new_bits)
        } else {
            None
        }
    }
}

fn read_block<T: Read>(mut reader: T) -> io::Result<Vec<DNA>> {
    let mut buffer = vec![0; INPUT_SIZE_BYTES];

    // Read file into vector.
    reader.read_exact(&mut buffer)?;
    let result = buffer
        .iter()
        .flat_map(dna::binary_to_DNA)
        .collect::<Vec<DNA>>();
    Ok(result)
}

fn write_block<T: Write>(mut writer: T, dna: Vec<DNA>) -> io::Result<usize> {
    let buffer = dna
        .chunks_exact(4)
        .map(|chunk| dna::DNA_to_binary(chunk.try_into().unwrap()))
        .collect::<Vec<u8>>();
    writer.write(&buffer)
}

fn u8_to_dna(input: [u8; INPUT_SIZE_BYTES]) -> Vec<DNA> {
    input.iter().flat_map(dnac::dna::binary_to_DNA).collect()
}

fn key_avalanche(output: File) -> io::Result<()> {
    let mut keys = BufReader::new(File::open(RAND_FILE)?);
    let mut buffer = [0; INPUT_SIZE_BYTES];
    let input_zeros = vec![DNA::A; 64];

    for _ in tqdm!(0..384) {
        if let Ok(_) = keys.read_exact(&mut buffer) {
            let key_0 = u8_to_dna(buffer);
            let cipher_0 = dnac::DNAC::new(key_0);
            let block_0 = cipher_0.encrypt(input_zeros.clone());
            for key in BitsOne::new(buffer) {
                let key = u8_to_dna(key);
                let cipher = dnac::DNAC::new(key);
                let block = cipher.encrypt(input_zeros.clone());
                let result: Vec<DNA> = block_0.iter().zip(block).map(|(&a, b)| a ^ b).collect();
                write_block(output.try_clone()?, result)?;
            }
        }
    }
    Ok(())
}

fn plaintext_avalanche(output: File) -> io::Result<()> {
    let mut texts = BufReader::new(File::open(RAND_FILE)?);
    let mut buffer = [0; INPUT_SIZE_BYTES];
    let cipher = dnac::DNAC::new(vec![DNA::A; 64]);

    for _ in tqdm!(0..384) {
        if let Ok(_) = texts.read_exact(&mut buffer) {
            let text_0 = u8_to_dna(buffer);
            let block_0 = cipher.encrypt(text_0);
            for text in BitsOne::new(buffer) {
                let text = u8_to_dna(text);
                let block = cipher.encrypt(text);
                let result: Vec<DNA> = block_0.iter().zip(block).map(|(&a, b)| a ^ b).collect();
                write_block(output.try_clone()?, result)?;
            }
        }
    }
    Ok(())
}

fn correlation(output: File) -> io::Result<()> {
    Ok(())
}

fn block_chaining(output: File) -> io::Result<()> {
    Ok(())
}

fn random(output: File) -> io::Result<()> {
    Ok(())
}

fn key_low_density(output: File) -> io::Result<()> {
    Ok(())
}

fn plaintext_low_density(output: File) -> io::Result<()> {
    Ok(())
}

fn key_high_density(output: File) -> io::Result<()> {
    Ok(())
}

fn plaintext_high_density(output: File) -> io::Result<()> {
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let output =
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(if let Some(file) = args.output {
                file
            } else {
                "resut.blb".to_string()
            })?;

    let cmd_map = {
        let mut map: HashMap<Commands, fn(File) -> io::Result<()>> = HashMap::new();
        map.insert(Commands::KeyAv, key_avalanche);
        map.insert(Commands::PlaintextAv, plaintext_avalanche);
        map.insert(Commands::Correlation, correlation);
        map.insert(Commands::BlockChaining, block_chaining);
        map.insert(Commands::Random, random);
        map.insert(Commands::KeyLD, key_low_density);
        map.insert(Commands::PlaintextLD, plaintext_low_density);
        map.insert(Commands::KeyHD, key_high_density);
        map.insert(Commands::PlaintextHD, plaintext_high_density);
        map
    };

    match args.command {
        Some(cmd) => cmd_map[&cmd](output),
        None => cmd_map
            .iter()
            .map(|(key, function)| {
                let output = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(key.to_string())?;
                function(output)
            })
            .collect(),
    }
}
