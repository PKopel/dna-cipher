use clap::{Parser, Subcommand};
use log::debug;
use std::fs::{File, OpenOptions};
use std::io;
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

fn read_key(mut file: File) -> io::Result<[DNA; feistel::KEY_SIZE]> {
    let mut buffer = [0u8; feistel::KEY_SIZE / 4];
    let bytes = file.read(&mut buffer)?;
    if bytes != feistel::KEY_SIZE / 4 {
        let msg = format!("wrong key size: expected 32B, got {}", bytes);
        return Err(io::Error::new(io::ErrorKind::Other, msg));
    };
    let result = buffer
        .iter()
        .flat_map(dna::binary_to_DNA)
        .collect::<Vec<DNA>>();
    return Ok(result.try_into().unwrap());
}

fn process<R, W, F>(mut reader: R, mut writer: W, fun: F) -> io::Result<()>
where
    R: Read,
    W: Write,
    F: Fn(Vec<DNA>) -> Result<Vec<DNA>, String>,
{
    let bytes = 2 * feistel::INPUT_CHUNK_SIZE;
    let mut buffer = Vec::with_capacity(bytes);
    loop {
        let length = reader
            .by_ref()
            .take(bytes as u64)
            .read_to_end(&mut buffer)?;
        if length == 0 {
            break;
        }
        let dna = buffer
            .iter()
            .flat_map(dna::binary_to_DNA)
            .collect::<Vec<DNA>>();
        match fun(dna) {
            Ok(result) => {
                let buffer = result
                    .chunks_exact(4)
                    .map(|chunk| dna::DNA_to_binary(chunk.try_into().unwrap()))
                    .collect::<Vec<u8>>();
                writer.write(&buffer)?;
            }
            Err(msg) => {
                return Err(io::Error::new(io::ErrorKind::Other, msg));
            }
        }
        buffer.clear();
    }
    return Ok(());
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    let key = match args.key {
        Some(file) => read_key(File::open(file)?)?,
        None => read_key(File::open("key.txt")?)?,
    };
    let reader: Box<dyn Read> = match args.input {
        Some(file) => Box::new(File::open(file)?),
        None => Box::new(stdin()),
    };
    let writer: Box<dyn Write> = match args.output {
        Some(file) => Box::new(OpenOptions::new().write(true).create(true).open(file)?),
        None => Box::new(stdout()),
    };

    debug!("key = {:?}", key);

    let fun = |dna: Vec<DNA>| -> Result<Vec<DNA>, String> {
        debug!("msg = {:?}", dna);
        if args.command == Commands::Encrypt {
            Ok(feistel::encrypt(dna, key))
        } else {
            feistel::decrypt(dna, key)
        }
    };

    return process(reader, writer, fun);
}
