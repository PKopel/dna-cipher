use clap::{Parser, Subcommand};
use log::{debug, error};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::BufReader;
use std::io::{stdin, stdout};
use std::io::{Read, Write};

pub mod dna;
pub mod sbox;
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
    #[arg(short, long, default_value_t = 0)]
    verbose: usize,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
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

    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let dna = match args.input {
        Some(file) => read_file(BufReader::new(File::open(file)?))?,
        None => read_file(stdin())?,
    };
    let key = match args.key {
        Some(file) => read_file(BufReader::new(File::open(file)?))?,
        None => read_file(stdin())?,
    };

    debug!("key = {:?}", key);
    debug!("msg = {:?}", dna);

    let result = if args.command == Commands::Encrypt {
        Ok(feistel::encrypt(dna, key))
    } else {
        feistel::decrypt(dna, key)
    };

    match result {
        Ok(result) => {
            debug!("enc = {:?}", result);

            match args.output {
                Some(file) => write_file(
                    OpenOptions::new().write(true).create(true).open(file)?,
                    result,
                ),
                None => write_file(stdout(), result),
            }?;
        }
        Err(msg) => error!("{}", msg),
    }

    return Ok(());
}
