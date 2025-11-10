use thiserror::Error;

use crate::token::TokenKind;

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Identifier and litterals tokens need a value to be instanciated.")]
    MissingTokenValue,

    #[error("Token kind {0} cannot have a value.")]
    InvalidTokenKind(TokenKind),
}
