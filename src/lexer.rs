use std::fmt::Display;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::{
        char,
        complete::{newline, space1},
    },
};

use thiserror::Error;

use crate::error::{NonError, Result};

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Litteral(String),
    Space,
    Dot,
    Colon,
    At,
    Newline,
    Eof,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(s) => f.write_str(&format!("Identifier {s}")),
            Self::Litteral(s) => f.write_str(&format!("Litteral {s}")),
            Self::Space => f.write_str("Space"),
            Self::Dot => f.write_str("Dot"),
            Self::Colon => f.write_str("Colon"),
            Self::At => f.write_str("At"),
            Self::Newline => f.write_str("NewLine"),
            Self::Eof => f.write_str("EOF"),
        }
    }
}

#[derive(Debug)]
pub struct NonLexer<'a> {
    remaining: &'a str,
}

impl<'a> NonLexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { remaining: source }
    }

    pub fn read_next_token(&mut self) -> Result<Token> {
        alt((
            parse_identifier,
            parse_litteral,
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
        .map_err(|_| NonError::TokenizeFailed)
    }
}

impl<'a> Iterator for NonLexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.read_next_token().ok()? {
                Token::Space => continue,
                token => return Some(token),
            }
        }
    }
}

fn parse_char_to_token(s: &str, c: char, token: Token) -> IResult<&str, Token> {
    char(c).parse(s).map(|(rest, _)| (rest, token))
}

fn parse_identifier(s: &str) -> IResult<&str, Token> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')
        .parse(s)
        .map(|(rest, id)| (rest, Token::Identifier(id.to_string())))
}

fn parse_litteral(s: &str) -> IResult<&str, Token> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identifier() {
        let s = "identifier";
        let (remaining, token) = parse_identifier(s).unwrap();

        assert_eq!(token, Token::Identifier("identifier".to_string()));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_litteral() {
        let s = "'value'";
        let (remaining, token) = parse_litteral(s).unwrap();

        assert_eq!(token, Token::Litteral("value".to_string()));
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_dot() {
        let s = ".";
        let (remaining, token) = parse_dot(s).unwrap();

        assert_eq!(token, Token::Dot);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_at() {
        let s = "@";
        let (remaining, token) = parse_at(s).unwrap();

        assert_eq!(token, Token::At);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_colon() {
        let s = ":";
        let (remaining, token) = parse_colon(s).unwrap();

        assert_eq!(token, Token::Colon);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_whitspace() {
        let s = " ";
        let (remaining, token) = parse_whitespace(s).unwrap();

        assert_eq!(token, Token::Space);
        assert_eq!(remaining, "");
    }

    #[test]
    fn test_parse_newline() {
        let s = "\n";
        let (remaining, token) = parse_newline(s).unwrap();

        assert_eq!(token, Token::Newline);
        assert_eq!(remaining, "");
    }
}
