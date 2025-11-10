use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use clap::Parser;

use crate::{
    args::{Args, OutputFormat},
    error::Result,
    lexer::NonLexer,
    parser::NonParser,
};

mod args;
mod error;
mod lexer;
mod non;
mod parser;
mod token;

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if path.exists() {
        let mut file = File::open(path)?;
        let mut buf = String::default();

        file.read_to_string(&mut buf)?;

        let lexer = NonLexer::new(&buf);
        let mut parser = NonParser::new(lexer);

        parser.parse();

        let non = parser.get_non_by_id("student").unwrap();
        let content = if let Some(format) = &args.format {
            match format {
                OutputFormat::Json => serde_json::to_string_pretty(&*non)?,
                OutputFormat::Yaml => serde_yaml::to_string(&*non)?,
                OutputFormat::Non => parser.serialize(),
            }
        } else {
            parser.serialize()
        };

        if let Some(output) = &args.output {
            let mut f = File::create(output)?;

            f.write_all(&content.into_bytes())?;
        } else {
            println!("{}", content);
        }
    }

    Ok(())
}
