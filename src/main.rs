use std::{fs::File, io::Read};

use crate::parser::{NonLexer, NonParser};

mod parser;

fn main() {
    let mut file = File::open("./test.non").unwrap();
    let mut buf = String::default();

    file.read_to_string(&mut buf).unwrap();

    let mut lexer = NonLexer::new(&buf);
    // println!("tokens: {:?}", lexer.read_all());

    let mut parser = NonParser::new(lexer);
    parser.parse();
}
