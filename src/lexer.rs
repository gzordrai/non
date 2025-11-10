use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::{
        char,
        complete::{newline, space1},
    },
};

use crate::token::{Token, TokenKind, TokenValue};

fn parse_char_to_token(s: &str, c: char, token: Token) -> IResult<&str, Token> {
    char(c).parse(s).map(|(rest, _)| (rest, token))
}

fn parse_identifier(s: &str) -> IResult<&str, Token> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')
        .parse(s)
        .map(|(rest, id)| {
            (
                rest,
                Token::new(TokenKind::Identifier, TokenValue::String(id.to_string())),
            )
        })
}

fn parse_string_litteral(s: &str) -> IResult<&str, Token> {
    (char('\''), take_while1(|c| c != '\''), char('\''))
        .parse(s)
        .map(|(rest, (_, s, _))| {
            (
                rest,
                Token::new(TokenKind::Litteral, TokenValue::String(s.to_string())),
            )
        })
}

fn parse_dot(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, '.', Token::from(TokenKind::Dot))
}

fn parse_at(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, '@', Token::from(TokenKind::At))
}

fn parse_colon(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, ':', Token::from(TokenKind::Colon))
}

fn parse_whitespace(s: &str) -> IResult<&str, Token> {
    space1(s).map(|(rest, _)| (rest, Token::from(TokenKind::Space)))
}

fn parse_newline(s: &str) -> IResult<&str, Token> {
    newline
        .parse(s)
        .map(|(rest, _)| (rest, Token::from(TokenKind::Newline)))
}

#[derive(Debug)]
pub struct NonLexer<'a> {
    remaining: &'a str,
}

#[allow(dead_code)]
impl<'a> NonLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { remaining: source }
    }

    pub fn read_next_token(&mut self) -> Result<Token, String> {
        alt((
            parse_identifier,
            parse_string_litteral,
            parse_whitespace,
            parse_dot,
            parse_colon,
            parse_at,
            parse_newline,
        ))
        .parse(self.remaining)
        .map(|(remaining, token)| {
            self.remaining = remaining;
            token
        })
        .map_err(|err| format!("Tokenize error : {}", err))
    }

    pub fn read_all(&mut self) -> Vec<Token> {
        self.into_iter().collect()
    }
}

impl<'a> Iterator for NonLexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_next_token().ok()
    }
}
