use clap::{Parser, Subcommand};
use dnac::{
    dna::{self, DNA},
    DNAC,
};
use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Write},
};

pub const INPUT_SIZE_BYTES: usize = 16;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long)]
    output: String,
    #[arg(short, long)]
    key: String,
    #[arg(short, long, default_value_t = 0)]
    verbose: usize,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
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

fn main() -> io::Result<()> {
    let args = Args::parse();

    let dna = BufReader::new(File::open(args.input)?);
    let key = BufReader::new(File::open(args.key)?);
    let output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(args.output)?;

    // let cipher = DNAC::new(key);

    match args.command {
        Commands::KeyAv => todo!(),
        Commands::PlaintextAv => todo!(),
        Commands::Correlation => todo!(),
        Commands::BlockChaining => todo!(),
        Commands::Random => todo!(),
        Commands::KeyLD => todo!(),
        Commands::PlaintextLD => todo!(),
        Commands::KeyHD => todo!(),
        Commands::PlaintextHD => todo!(),
    }
}
