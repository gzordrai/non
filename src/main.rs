use std::{fs::File, io::Read};

use crate::{lexer::NonLexer, parser::NonParser};

mod error;
mod lexer;
mod non;
mod parser;
mod token;

fn main() {
    let mut file = File::open("./test.non").unwrap();
    let mut buf = String::default();

    file.read_to_string(&mut buf).unwrap();

    let lexer = NonLexer::new(&buf);
    // println!("tokens: {:?}", lexer.read_all());

    let mut parser = NonParser::new(lexer);
    parser.parse();
    parser.resolve_all();
    for (id, non) in &parser.noms {
        println!("{}: {:?}\n\n", id, non);
    }
}
