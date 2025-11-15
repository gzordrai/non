use thiserror::Error;

use crate::token::Token;

pub type Result<T> = std::result::Result<T, NonError>;

#[derive(Debug, Error)]
pub enum NonError {
    #[error("Tokenize error")]
    TokenizeFailed,

    #[error("Identifier and litterals tokens need a value to be instanciated.")]
    MissingTokenValue,

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Parsing error")]
    ParsingError,

    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Unexpected identifier: {0}")]
    UnexpectedIdentifier(String),

    #[error("Empty field value")]
    EmptyFieldValue,
}
