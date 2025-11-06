#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Litteral,
    Space,
    Dot,
    Colon,
    At,
    Newline,
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenValue {
    None,
    String(String),
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

    pub fn get_token_str_raw_value(&self) -> String {
        match self.kind {
            TokenKind::Identifier | TokenKind::Litteral => {
                if let TokenValue::String(value) = &self.value {
                    value.to_owned()
                } else {
                    panic!("No value found in identifier or litteral.")
                }
            }
            _ => {
                panic!(
                    "{}",
                    format!("Token kind {:?} cannot have a value.", self.kind)
                )
            }
        }
    }

    pub fn from_kind(kind: TokenKind) -> Self {
        Token::verify_token(kind, TokenValue::None);
        Token::new(kind, TokenValue::None)
    }

    fn verify_token(kind: TokenKind, value: TokenValue) {
        match kind {
            TokenKind::Identifier | TokenKind::Litteral => {
                if let TokenValue::None = value {
                    panic!("Identifier and litterals tokens need a value to be instanciated.")
                }
            }
            _ => {
                if let TokenValue::String(_) = value {
                    panic!("{}", format!("Token kind {:?} cannot have a value.", kind))
                }
            }
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::EOF,
            value: TokenValue::None,
        }
    }
}
