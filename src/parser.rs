use nom::{
    IResult, Needed, Parser,
    bytes::complete::tag,
    character::{
        char,
        complete::{alpha1, newline},
    },
    multi::{many, many1},
};
use std::{collections::HashMap, str::FromStr};

fn parse_newline(r: &str) -> IResult<&str, &str> {
    newline(r).and_then(|(r, _)| Ok((r, "")))
}

fn parse_identifier(s: &str) -> IResult<&str, Token> {
    (alpha1, char(':'), parse_newline)
        .parse(s)
        .map(|(rest, (id, _, _))| (rest, Token::Identifier(id.to_string())))
}

fn parse_field(s: &str) -> IResult<&str, Token> {
    (char('.'), alpha1, char(' '), alpha1)
    .parse(s)
    .map(|(rest, (_, name, _, value)| (rest, Token::Litteral(value))))
}

#[derive(Debug)]
pub struct Tokenizer {
    tokens: Vec<Token>,
}

impl FromStr for Tokenizer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = vec![];
        let (mut rest, identifier) = parse_identifier(s).unwrap();
        println!("rest: {rest}");
        tokens.push(identifier);

        rest = rest.trim();

        let (rest, field) = parse_field(rest).unwrap();

        println!("ok {:?}", tag("aa").parse("aa") as IResult<&str, &str>);

        println!("{tokens:?}");

        Ok(Tokenizer { tokens })
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Litteral(String),
    Identifier(String),
    Ref(String),
    ObjRef(String, String),
}

#[derive(Debug, Default)]
pub struct Non {
    fields: HashMap<String, String>,
}

impl Non {
    pub fn new() -> Self {
        Self::default()
    }
}

// impl From<Path> for Non {
//     fn from(value: Path) -> Self {
//         todo!()
//     }
// }
