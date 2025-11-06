use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::take_while1,
    character::{
        char,
        complete::{newline, space1},
    },
};
use std::collections::HashMap;

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
    parse_char_to_token(s, '.', Token::from_kind(TokenKind::Dot))
}

fn parse_at(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, '@', Token::from_kind(TokenKind::At))
}

fn parse_colon(s: &str) -> IResult<&str, Token> {
    parse_char_to_token(s, ':', Token::from_kind(TokenKind::Colon))
}

fn parse_whitespace(s: &str) -> IResult<&str, Token> {
    space1(s).map(|(rest, _)| (rest, Token::from_kind(TokenKind::Space)))
}

fn parse_newline(s: &str) -> IResult<&str, Token> {
    newline
        .parse(s)
        .map(|(rest, _)| (rest, Token::from_kind(TokenKind::Newline)))
}

#[derive(Debug)]
pub struct NonLexer<'a> {
    remaining: &'a str,
}

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
    kind: TokenKind,
    value: TokenValue,
}

impl Token {
    pub fn new(kind: TokenKind, value: TokenValue) -> Self {
        Self { kind, value }
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

    fn get_token_str_raw_value(&self) -> String {
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
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::EOF,
            value: TokenValue::None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Non {
    id: String,
    fields: HashMap<String, FieldValue>,
    parent: Option<String>,
}

impl Non {
    pub fn from_id(id: String) -> Self {
        Non {
            id,
            ..Default::default()
        }
    }

    pub fn add_field(&mut self, name: String, value: FieldValue) {
        self.fields.insert(name, value);
    }
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Litteral(String),
    Vec(Vec<FieldValue>),
    FieldReference(String),
    ObjRef(String, String),
}

#[derive(Debug)]
pub struct NonParser<'a> {
    current_token: Token,
    lexer: NonLexer<'a>,
    pub noms: HashMap<String, Non>,
}

impl<'a> NonParser<'a> {
    pub fn new(lexer: NonLexer<'a>) -> Self {
        Self {
            current_token: Token::default(),
            noms: HashMap::new(),
            lexer,
        }
    }

    pub fn parse(&mut self) {
        self.advance();
        self.skip_spaces_and_newlines();
        if self.is_kind(TokenKind::Identifier) {
            self.parse_non();
        }
    }

    fn parse_non(&mut self) {
        println!("ident {:?}", self.current_token);
        let mut non = Non::from_id(self.current_token.get_token_str_raw_value());

        self.advance();

        if !self.eat(TokenKind::Colon) {
            panic!("Error parsing");
        }

        self.skip_spaces();

        if self.is_kind(TokenKind::Identifier) {
            non.parent = Some(self.current_token.get_token_str_raw_value());
            self.advance();
        }

        if !self.is_kind(TokenKind::Newline) {
            panic!("Error parsing");
        }

        println!("non {:?}", non);
    }

    fn skip_spaces(&mut self) {
        loop {
            if !self.is_kind(TokenKind::Space) {
                break;
            }
            self.advance();
        }
    }

    fn skip_newlines(&mut self) {
        loop {
            if !self.is_kind(TokenKind::Newline) {
                break;
            }
            self.advance();
        }
    }

    fn skip_spaces_and_newlines(&mut self) {
        loop {
            if !(self.is_kind(TokenKind::Space) || self.is_kind(TokenKind::Newline)) {
                break;
            }
            self.advance();
        }
    }

    fn current_token(&self) -> &Token {
        &self.current_token
    }

    fn current_kind(&self) -> TokenKind {
        self.current_token.kind
    }

    fn is_kind(&self, kind: TokenKind) -> bool {
        self.current_kind() == kind
    }

    fn eat(&mut self, kind: TokenKind) -> bool {
        if self.is_kind(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    // fn eat(&mut self, kind: TokenKind) -> bool {
    //     self.advance();
    //     self.is_kind(kind)
    // }

    fn advance(&mut self) {
        self.current_token = self.lexer.read_next_token().unwrap_or(Token::default());
        println!("cur {:?}", self.current_token);
    }
}
