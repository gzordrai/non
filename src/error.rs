use thiserror::Error;

use crate::lexer::Token;

pub type Result<T> = std::result::Result<T, NonError>;

#[derive(Debug, Error)]
pub enum NonError {
    #[error("Tokenize error")]
    TokenizeFailed,

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Unexpected token: {0}")]
    UnexpectedToken(Token),

    #[error("Unexpected EOF")]
    UnexpectedEof,

    #[error("Unexpected identifier: {0}")]
    UnexpectedIdentifier(String),

    #[error("Empty field value")]
    EmptyFieldValue,

    #[error("Cyclic dependency found: {0}")]
    CyclicDependency(String),

    #[error("Undefined non: {0}")]
    UndefinedNon(String),
}
