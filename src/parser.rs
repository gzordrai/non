use nom::{
    IResult, Needed, Parser,
    branch::alt,
    bytes::complete::{take_while, take_while1},
    character::{
        char,
        complete::{alpha1, newline, space1},
    },
    combinator::complete,
    multi::{many, many0, many1},
};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

fn parse_char_to_token(s: &str, c: char, token: Token) -> IResult<&str, Token> {
    char(c).parse(s).map(|(rest, _)| (rest, token))
}

fn parse_identifier(s: &str) -> IResult<&str, Token> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')
        .parse(s)
        .map(|(rest, id)| (rest, Token::Identifier(id.to_string())))
}

fn parse_string_litteral(s: &str) -> IResult<&str, Token> {
    (char('\''), take_while1(|c| c != '\''), char('\''))
        .parse(s)
        .map(|(rest, (_, s, _))| (rest, Token::Litteral(s.to_string())))
}

fn parse_dot(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, '.', Token::Dot)
}

fn parse_at(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, '@', Token::At)
}

fn parse_colon(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, ':', Token::Colon)
}

fn parse_whitespace(s: &str) -> IResult<&str, Token> {
    space1(s).map(|(rest, _)| (rest, Token::Space))
}

fn parse_newline(s: &str) -> IResult<&str, Token> {
    newline.parse(s).map(|(rest, _)| (rest, Token::Newline))
}

#[derive(Debug)]
pub struct Tokenizer {
    pub tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
}

impl FromStr for Tokenizer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, tokens) = many0(complete(alt((
            parse_identifier,
            parse_string_litteral,
            parse_whitespace,
            parse_dot,
            parse_colon,
            parse_at,
            parse_newline,
        ))))
        .parse(s)
        .map_err(|err| err.to_string())?;

        if !rest.is_empty() {
            Err(format!("Error parsing file, rest: {rest}"))
        } else {
            Ok(Tokenizer::new(tokens))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Litteral(String),
    Space,
    Dot,
    Colon,
    At,
    Newline,
}

#[derive(Debug, Default, Clone)]
pub struct Non {
    id: String,
    fields: HashMap<String, FieldValue>,
    parent: Option<Box<Non>>,
}

#[derive(Debug, Clone)]
enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(String, String),
}

impl Non {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub tokens: VecDeque<Token>,
    pub noms: HashMap<String, Non>,
}

impl Lexer {
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self {
            tokens: VecDeque::from(tokenizer.tokens),
            noms: HashMap::new(),
        }
    }
}
