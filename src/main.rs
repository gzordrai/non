use std::{fs::File, io::Read, path::Path};

use clap::Parser;

use crate::{
    args::{Args, OutputFormat},
    lexer::NonLexer,
    parser::NonParser,
};

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
        let mut parser = NonParser::new(lexer);

        parser.parse();

        let non = parser.at("alice").unwrap();

        if let Some(format) = &args.format {
            let content = match format {
                OutputFormat::Json => non.serialize_json(true),
                OutputFormat::Yaml => non.serialize_yaml(true),
                OutputFormat::Non => parser.serialize(),
            };

            println!("{}", content);
        }
        println!("alice.mail {}", non.get("mail").unwrap());
    }

}
