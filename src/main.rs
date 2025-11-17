use std::{fs::File, io::Read, path::Path, str::FromStr};

use clap::Parser;

// use crate::{args::Args, lexer::NonLexer, nds::NonDefs, parser::NonParser};
use crate::{args::Args, error::Result, lexer::NonLexer, nds::NonDefs, parser::NonParser};

mod args;
mod error;
mod lexer;
mod nds;
mod non;
mod parser;

fn main() -> Result<()> {
    // let args = Args::parse();
    // let path = Path::new(&args.path);

    // if path.exists() {
    let mut file = File::open("./test.non")?;
    let mut buf = String::default();

    file.read_to_string(&mut buf)?;

    let nds = NonDefs::from_str(&buf)?;
    let a = nds.at("alice").unwrap();

    println!("{}", a.id());
    println!("{:?}", a.get("name"));
    println!("{:?}", a.get("login"));
    println!("{:?}", a.get("mail"));

    let b = nds.at("bob").unwrap();

    println!("{:?}", b.get("login"));
    println!("{:?}", b.get("name"));
    println!("{:?}", b.get("mail"));

    println!("{:?}", nds.at("univ").unwrap().get("login"));
    println!("{:?}", nds.at("foo"));

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
    // }

    Ok(())
}
