use clap::{Parser, Subcommand};
use dnac::{
    bits::{BitsOne, BitsTwo},
    dna::{self, DNA},
    DNAC,
};
use kdam::tqdm;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Write},
};

const INPUT_SIZE_BYTES: usize = 16;
const INPUT_SIZE_DNA: usize = 64;
const RAND_FILE: &str = "/dev/urandom";
// const ZERO_FILE: &str = "/dev/zero";

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

fn write_block<T: Write>(mut writer: T, dna: &Vec<DNA>) -> io::Result<usize> {
    let buffer = dna
        .chunks_exact(4)
        .map(|chunk| dna::DNA_to_binary(chunk.try_into().unwrap()))
        .collect::<Vec<u8>>();
    writer.write(&buffer)
}

fn u8_to_dna(input: [u8; INPUT_SIZE_BYTES]) -> Vec<DNA> {
    input.iter().flat_map(dna::binary_to_DNA).collect()
}

fn key_avalanche(output: File) -> io::Result<()> {
    let mut keys = BufReader::new(File::open(RAND_FILE)?);
    let mut buffer = [0; INPUT_SIZE_BYTES];
    let input_zeros = vec![DNA::A; INPUT_SIZE_DNA];

    for _ in tqdm!(0..24576) {
        if let Ok(_) = keys.read_exact(&mut buffer) {
            let key_0 = u8_to_dna(buffer);
            let cipher_0 = DNAC::new(key_0);
            let block_0 = cipher_0.encrypt(input_zeros.clone());
            for key in BitsOne::new(buffer) {
                let key = u8_to_dna(key);
                let cipher = DNAC::new(key);
                let block = cipher.encrypt(input_zeros.clone());
                let result: Vec<DNA> = block_0.iter().zip(block).map(|(&a, b)| a ^ b).collect();
                write_block(output.try_clone()?, &result)?;
            }
        }
    }
    Ok(())
}

fn plaintext_avalanche(output: File) -> io::Result<()> {
    let mut texts = BufReader::new(File::open(RAND_FILE)?);
    let mut buffer = [0; INPUT_SIZE_BYTES];
    let cipher = DNAC::new(vec![DNA::A; INPUT_SIZE_DNA]);

    for _ in tqdm!(0..24576) {
        if let Ok(_) = texts.read_exact(&mut buffer) {
            let text_0 = u8_to_dna(buffer);
            let block_0 = cipher.encrypt(text_0);
            for text in BitsOne::new(buffer) {
                let text = u8_to_dna(text);
                let block = cipher.encrypt(text);
                let result = block_0.iter().zip(block).map(|(&a, b)| a ^ b).collect();
                write_block(output.try_clone()?, &result)?;
            }
        }
    }
    Ok(())
}

fn correlation(output: File) -> io::Result<()> {
    const INPUT_BLOCKS_SIZE: usize = 130048; // 16 (one block in bytes) * 8128
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);
    let mut texts = [0; INPUT_BLOCKS_SIZE];
    inputs.read_exact(&mut texts)?;

    for _ in tqdm!(0..128) {
        let mut key = [0; INPUT_SIZE_BYTES];
        inputs.read_exact(&mut key)?;
        let key = u8_to_dna(key);
        let cipher = DNAC::new(key);
        for text in texts.chunks_exact(INPUT_SIZE_BYTES) {
            let text = u8_to_dna(text.try_into().unwrap());
            let block = cipher.encrypt(text.clone());
            let result = text.iter().zip(block).map(|(&a, b)| a ^ b).collect();
            write_block(output.try_clone()?, &result)?;
        }
    }
    Ok(())
}

fn block_chaining(output: File) -> io::Result<()> {
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);
    let text = vec![DNA::A; INPUT_SIZE_DNA];

    for _ in tqdm!(0..300) {
        let mut iv = [0; INPUT_SIZE_BYTES];
        let mut key = [0; INPUT_SIZE_BYTES];
        inputs.read_exact(&mut iv)?;
        inputs.read_exact(&mut key)?;
        let mut iv = u8_to_dna(iv);
        let key = u8_to_dna(key);
        let cipher = DNAC::new(key);
        for _ in 0..8192 {
            let input: Vec<DNA> = text.iter().zip(iv.clone()).map(|(&a, b)| a ^ b).collect();
            iv = cipher.encrypt(input.clone());
            write_block(output.try_clone()?, &iv)?;
        }
    }
    Ok(())
}

fn random(output: File) -> io::Result<()> {
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);

    for _ in tqdm!(0..128) {
        let mut key = [0; INPUT_SIZE_BYTES];
        inputs.read_exact(&mut key)?;
        let key = u8_to_dna(key);
        let cipher = DNAC::new(key);
        for _ in 0..8128 {
            let mut text = [0; INPUT_SIZE_BYTES];
            inputs.read_exact(&mut text)?;
            let text = u8_to_dna(text);
            let block = cipher.encrypt(text.clone());
            write_block(output.try_clone()?, &block)?;
        }
    }
    Ok(())
}

fn key_low_density(output: File) -> io::Result<()> {
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);

    for _ in tqdm!(0..128) {
        let keys = [[0; INPUT_SIZE_BYTES]]
            .iter()
            .map(|v| v.to_owned())
            .chain(BitsOne::new([0; INPUT_SIZE_BYTES]))
            .chain(BitsTwo::new([0; INPUT_SIZE_BYTES]));

        for key in keys {
            let key = u8_to_dna(key);
            let cipher = DNAC::new(key);

            let mut text = [0; INPUT_SIZE_BYTES];
            inputs.read_exact(&mut text)?;
            let text = u8_to_dna(text);
            let block = cipher.encrypt(text.clone());
            write_block(output.try_clone()?, &block)?;
        }
    }
    Ok(())
}

fn plaintext_low_density(output: File) -> io::Result<()> {
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);

    for _ in tqdm!(0..128) {
        let mut key = [0; INPUT_SIZE_BYTES];
        inputs.read_exact(&mut key)?;
        let key = u8_to_dna(key);
        let cipher = DNAC::new(key);

        let texts = [[0; INPUT_SIZE_BYTES]]
            .iter()
            .map(|v| v.to_owned())
            .chain(BitsOne::new([0; INPUT_SIZE_BYTES]))
            .chain(BitsTwo::new([0; INPUT_SIZE_BYTES]));

        for text in texts {
            let text = u8_to_dna(text);
            let block = cipher.encrypt(text.clone());
            write_block(output.try_clone()?, &block)?;
        }
    }
    Ok(())
}

fn key_high_density(output: File) -> io::Result<()> {
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);

    for _ in tqdm!(0..128) {
        let keys = [[0b1111_1111; INPUT_SIZE_BYTES]]
            .iter()
            .map(|v| v.to_owned())
            .chain(BitsOne::new([0b1111_1111; INPUT_SIZE_BYTES]))
            .chain(BitsTwo::new([0b1111_1111; INPUT_SIZE_BYTES]));

        for key in keys {
            let key = u8_to_dna(key);
            let cipher = DNAC::new(key);

            let mut text = [0; INPUT_SIZE_BYTES];
            inputs.read_exact(&mut text)?;
            let text = u8_to_dna(text.try_into().unwrap());
            let block = cipher.encrypt(text.clone());
            write_block(output.try_clone()?, &block)?;
        }
    }
    Ok(())
}

fn plaintext_high_density(output: File) -> io::Result<()> {
    let mut inputs = BufReader::new(File::open(RAND_FILE)?);

    for _ in tqdm!(0..128) {
        let mut key = [0; INPUT_SIZE_BYTES];
        inputs.read_exact(&mut key)?;
        let key = u8_to_dna(key);
        let cipher = DNAC::new(key);

        let texts = [[0b1111_1111; INPUT_SIZE_BYTES]]
            .iter()
            .map(|v| v.to_owned())
            .chain(BitsOne::new([0b1111_1111; INPUT_SIZE_BYTES]))
            .chain(BitsTwo::new([0b1111_1111; INPUT_SIZE_BYTES]));

        for text in texts {
            let text = u8_to_dna(text);
            let block = cipher.encrypt(text.clone());
            write_block(output.try_clone()?, &block)?;
        }
    }
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
