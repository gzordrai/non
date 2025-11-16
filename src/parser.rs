use std::{collections::HashMap, iter::Peekable};

use crate::{
    error::{NonError, Result},
    lexer::{NonLexer, Token},
    non::Non,
};

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    String(String),           // 'alice'
    Reference,                // @
    SelfFieldRef(String),     // .login (reference to own field)
    FieldRef(String, String), // univ.domain (non.field)
    Concat(Vec<FieldValue>),  // multiple values concatenated
}

#[derive(Debug)]
pub struct NonParser<'a> {
    lexer: Peekable<NonLexer<'a>>,
}

impl<'a> NonParser<'a> {
    pub fn new(lexer: NonLexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    fn advance(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    fn expect(&mut self, expected: Token) -> Result<()> {
        match self.advance() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(NonError::UnexpectedToken(token)),
            None => Err(NonError::UnexpectedEof),
        }
    }

    pub fn parse(&mut self) -> Result<HashMap<String, Non>> {
        let mut nons: HashMap<String, Non> = HashMap::default();

        while self.peek().is_some() {
            let non = self.parse_non()?;

            println!("{:?}", non);
            nons.insert(non.name.clone(), non);
        }

        Ok(nons)
    }

    fn parse_non(&mut self) -> Result<Non> {
        while matches!(self.peek(), Some(Token::Newline)) {
            self.advance();
        }

        let id = match self.advance() {
            Some(Token::Identifier(id)) => id,
            Some(token) => return Err(NonError::UnexpectedToken(token)),
            None => return Err(NonError::UnexpectedEof),
        };

        self.expect(Token::Colon)?;

        let parents = self.parse_parents()?;

        self.expect(Token::Newline)?;

        let fields = self.parse_fields()?;

        Ok(Non::new(id, parents, fields))
    }

    fn parse_parents(&mut self) -> Result<Vec<String>> {
        let mut parents = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Identifier(_) => {
                    if let Some(Token::Identifier(id)) = self.advance() {
                        parents.push(id);
                    }
                }
                Token::Newline => break,
                token => return Err(NonError::UnexpectedToken(token.clone())),
            }
        }

        Ok(parents)
    }

    fn parse_fields(&mut self) -> Result<Vec<Field>> {
        let mut fields = Vec::new();

        while matches!(self.peek(), Some(Token::Dot)) {
            fields.push(self.parse_field()?);

            self.expect(Token::Newline)?;
        }

        Ok(fields)
    }

    fn parse_field(&mut self) -> Result<Field> {
        self.expect(Token::Dot)?;

        let name = match self.advance() {
            Some(Token::Identifier(id)) => id,
            Some(token) => return Err(NonError::UnexpectedToken(token)),
            None => return Err(NonError::UnexpectedEof),
        };

        let value = self.parse_field_value()?;

        Ok(Field { name, value })
    }

    fn parse_field_value(&mut self) -> Result<FieldValue> {
        let mut values = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Newline => break,
                Token::Litteral(_) => {
                    if let Some(Token::Litteral(s)) = self.advance() {
                        values.push(FieldValue::String(s));
                    }
                }
                Token::At => {
                    self.advance();
                    values.push(FieldValue::Reference);
                }
                Token::Dot => {
                    self.advance();

                    let field_ref = self.parse_field_reference()?;

                    values.push(field_ref);
                }
                Token::Identifier(id) => {
                    let id = id.clone();

                    self.advance();

                    if matches!(self.peek(), Some(Token::Dot)) {
                        self.advance();

                        if let Some(Token::Identifier(field)) = self.advance() {
                            values.push(FieldValue::FieldRef(id, field));
                        }
                    } else {
                        return Err(NonError::UnexpectedIdentifier(id));
                    }
                }
                token => return Err(NonError::UnexpectedToken(token.clone())),
            }
        }

        match values.len() {
            0 => Err(NonError::EmptyFieldValue),
            1 => Ok(values.into_iter().next().unwrap()),
            _ => Ok(FieldValue::Concat(values)),
        }
    }

    fn parse_field_reference(&mut self) -> Result<FieldValue> {
        match self.advance() {
            Some(Token::Identifier(field)) => Ok(FieldValue::SelfFieldRef(field)),
            Some(token) => Err(NonError::UnexpectedToken(token)),
            None => Err(NonError::UnexpectedEof),
        }
    }
}
