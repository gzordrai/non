use std::{fs::File, io::Read, path::Path};

use clap::Parser;

use crate::{
    args::Args, lexer::NonLexer, nds::NonDefs, parser::NonParser
};

mod args;
mod error;
mod lexer;
mod non;
mod parser;
mod token;
mod nds;

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
        let non_defs = NonDefs::builder()
            .format(args.format)
            .nons(parser.nons)
            .flat(args.flat)
            .build();
        
        let content = non_defs.serialize();
        println!("{}", content);

        let _ = non_defs.at("alice").unwrap();
        // println!("alice.mail {}", non.get("mail").unwrap());
    }

}
