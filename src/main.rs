use std::{fs::File, io::Read, str::FromStr};

use crate::parser::Tokenizer;

mod parser;

fn main() {
    let mut file = File::open("./test.non").unwrap();
    let mut buf = String::default();

    file.read_to_string(&mut buf).unwrap();

    Tokenizer::from_str(&buf).unwrap();
}
