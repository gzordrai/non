use std::{fs::File, io::Read, path::Path};

use clap::Parser;

// use crate::{args::Args, lexer::NonLexer, nds::NonDefs, parser::NonParser};
use crate::{args::Args, error::Result, lexer::NonLexer, parser::NonParser};

mod args;
mod error;
mod lexer;
// mod nds;
mod non;
mod parser;

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if path.exists() {
        let mut file = File::open(path)?;
        let mut buf = String::default();

        file.read_to_string(&mut buf)?;

        let lexer = NonLexer::new(&buf);
        let mut parser = NonParser::new(lexer);

        parser.parse()?;

        // let non_defs = NonDefs::builder()
        //     .format(args.format)
        //     .nons(parser.nons)
        //     .flat(args.flat)
        //     .build();

        // let content = non_defs.serialize();
        // println!("{}", content);

        // let alice = non_defs.at("alice").unwrap();
        // let bob = non_defs.at("bob").unwrap();
        // println!("alice.mail {}", alice.get("mail").unwrap());

        // let b = non_defs.at("b").unwrap();
        // let c = non_defs.at("c").unwrap();
        // let union = b.union(c);
        // println!("union : {:?}", union);
    }

    Ok(())
}
