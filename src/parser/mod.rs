use std::fs;
use std::io::{BufReader, Read};

pub mod env;
pub mod cursor;
pub mod k;
pub mod state;

mod parse_bytes;
use parse_bytes::parse_bytes;

pub fn parse(filepath: &std::path::PathBuf) -> env::File {
  let file = fs::File::open(filepath).unwrap();
  let mut reader = BufReader::new(file);
  let mut bytes: Vec<u8> = Vec::with_capacity(1024);

  reader.read_to_end(&mut bytes).unwrap();

  env::File {
    errors: Vec::new(),
    body: parse_bytes(&bytes),
  }
}