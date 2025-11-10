use std::fmt::Display;

use thiserror::Error;

use crate::error::{LexerError, Result};

pub type TokenValue = Option<String>;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Litteral,
    Space,
    Dot,
    Colon,
    At,
    Newline,
    Eof,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier => f.write_str("Identifier"),
            Self::Litteral => f.write_str("Litteral"),
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
pub struct Token {
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl Token {
    pub fn new(kind: TokenKind, value: TokenValue) -> Self {
        Self { kind, value }
    }

    pub fn get_token_str_raw_value(&self) -> Option<String> {
        if matches!(self.kind, TokenKind::Identifier | TokenKind::Litteral)
            && let Some(value) = &self.value
        {
            return Some(value.to_owned());
        }

        None
    }

    fn verify_token(kind: TokenKind, value: TokenValue) -> Result<()> {
        if matches!(kind, TokenKind::Identifier | TokenKind::Litteral) {
            if value.is_some() {
                return Ok(());
            } else {
                return Err(LexerError::MissingTokenValue);
            }
        }

        Err(LexerError::InvalidTokenKind(kind))
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::Eof,
            value: None,
        }
    }
}

impl From<TokenKind> for Token {
    fn from(token: TokenKind) -> Self {
        let _ = Token::verify_token(token, None);
        Token::new(token, None)
    }
}
