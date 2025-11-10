use thiserror::Error;

use crate::token::TokenKind;

pub type Result<T> = std::result::Result<T, NonError>;

#[derive(Debug, Error)]
pub enum NonError {
    #[error("Tokenize error")]
    TokenizeFailed,

    #[error("Identifier and litterals tokens need a value to be instanciated.")]
    MissingTokenValue,

    #[error("Token kind {0} cannot have a value.")]
    InvalidTokenKind(TokenKind),

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Json serialization error")]
    JsonSerializationError(#[from] serde_json::Error),

    #[error("Json serialization error")]
    YamlSerializationError(#[from] serde_yaml::Error),
}
