use clap::{Parser, Subcommand};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::{stdin, stdout};
use std::io::{Read, Write};

pub mod dna;
use dna::DNA;

pub mod feistel;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    output: Option<String>,
    #[arg(short, long)]
    key: Option<String>,
    #[arg(short, long)]
    debug: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Encrypt,
    Decrypt,
}

fn read_file<T: Read>(mut reader: T) -> io::Result<Vec<DNA>> {
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer)?;
    let result = buffer
        .iter()
        .flat_map(dna::binary_to_DNA)
        .collect::<Vec<DNA>>();
    return Ok(result);
}

fn write_file<T: Write>(mut writer: T, dna: Vec<DNA>) -> io::Result<usize> {
    let buffer = dna
        .chunks_exact(4)
        .map(|chunk| dna::DNA_to_binary(chunk.try_into().unwrap()))
        .collect::<Vec<u8>>();
    return writer.write(&buffer);
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut dna = match args.input {
        Some(file) => read_file(BufReader::new(File::open(file)?))?,
        None => read_file(stdin())?,
    };
    let key = match args.key {
        Some(file) => read_file(BufReader::new(File::open(file)?))?,
        None => read_file(stdin())?,
    };
    if args.debug {
        println!("{:?}", key);
        println!("{:?}", dna);
    }

    for _ in 0..3 {
        let (mut h, mut t) = feistel::round(dna, key.clone());
        if args.debug {
            println!("{:?}{:?}", h, t);
        }
        t.append(&mut h);
        dna = t
    }

    match args.output {
        Some(file) => write_file(File::open(file)?, dna),
        None => write_file(stdout(), dna),
    }?;

    return Ok(());
}
