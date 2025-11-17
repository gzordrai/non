use std::{
    collections::{HashMap, HashSet},
    iter::Peekable,
};

use crate::{
    error::{NonError, Result},
    lexer::{NonLexer, Token},
    nds::NonDefs,
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

    #[inline]
    fn peek(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    #[inline]
    fn advance(&mut self) -> Option<Token> {
        self.lexer.next()
    }

    #[inline]
    fn expect(&mut self, expected: Token) -> Result<()> {
        match self.advance() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(NonError::UnexpectedToken(token)),
            None => Err(NonError::UnexpectedEof),
        }
    }

    pub fn parse(&mut self) -> Result<NonDefs> {
        let mut nons: HashMap<String, Non> = HashMap::default();

        while self.peek().is_some() {
            let non = self.parse_non()?;

            nons.insert(non.name.clone(), non);
        }

        Self::detect_cycles(&nons)?;

        Ok(NonDefs::new(nons))
    }

    #[inline]
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

    fn detect_cycles(nons: &HashMap<String, Non>) -> Result<()> {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        for name in nons.keys() {
            if Self::has_cycle(name, nons, &mut visited, &mut stack)? {
                return Err(NonError::CyclicDependency(name.clone()));
            }
        }

        Ok(())
    }

    fn has_cycle(
        name: &str,
        nons: &HashMap<String, Non>,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
    ) -> Result<bool> {
        if stack.contains(name) {
            return Ok(true);
        }

        if visited.contains(name) {
            return Ok(false);
        }

        let non = nons
            .get(name)
            .ok_or_else(|| NonError::UndefinedNon(name.to_string()))?;

        visited.insert(name.to_string());
        stack.insert(name.to_string());

        for parent in &non.parents {
            if Self::has_cycle(parent, nons, visited, stack)? {
                return Ok(true);
            }
        }

        stack.remove(name);

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn cyclic_definition() {
        let a = "a".to_string();
        let b = "b".to_string();
        let c = "c".to_string();
        let a_def = Non {
            name: a.clone(),
            parents: vec![],
            fields: vec![Field {
                name: a.clone(),
                value: FieldValue::String(a.clone()),
            }],
        };
        let b_def = Non {
            name: b.clone(),
            parents: vec![a.clone(), c.clone()],
            fields: vec![Field {
                name: b.clone(),
                value: FieldValue::String(b.clone()),
            }],
        };
        let c_def = Non {
            name: c.clone(),
            parents: vec![b.clone()],
            fields: vec![Field {
                name: c.clone(),
                value: FieldValue::String(c.clone()),
            }],
        };
        let mut nons: HashMap<String, Non> = HashMap::default();

        nons.insert(a, a_def);
        nons.insert(b, b_def);
        nons.insert(c, c_def);

        NonParser::detect_cycles(&nons).unwrap();
    }
}
