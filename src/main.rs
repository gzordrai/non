use std::{fs::File, io::Read, path::Path};

use clap::Parser;

use crate::{args::Args, lexer::NonLexer, parser::NonParser};

mod args;
mod error;
mod lexer;
mod non;
mod parser;
mod token;

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if path.exists() {
        let mut file = File::open(path).unwrap();
        let mut buf = String::default();

        file.read_to_string(&mut buf).unwrap();

        let lexer = NonLexer::new(&buf);
        // println!("tokens: {:?}", lexer.read_all());

        let mut parser = NonParser::new(lexer);
        parser.parse();
        // parser.resolve_all();

        // println!("{}", parser.serialize());
        let non = parser.get_non_by_id("student").unwrap();
        println!("{}", non.serialize())
    }
}
