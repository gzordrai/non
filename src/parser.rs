use std::{collections::HashMap, rc::Rc};

use crate::{
    lexer::NonLexer,
    non::{FieldValue, Non},
    token::{Token, TokenKind},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct NonParser<'a> {
    current_token: Token,
    lexer: NonLexer<'a>,
    missing: HashMap<String, Rc<Non>>,
    pub noms: HashMap<String, Rc<Non>>,
}

#[allow(dead_code)]
impl<'a> NonParser<'a> {
    pub fn new(lexer: NonLexer<'a>) -> Self {
        Self {
            current_token: Token::default(),
            noms: HashMap::new(),
            missing: HashMap::new(),
            lexer,
        }
    }

    pub fn parse(&mut self) {
        self.advance();
        self.skip_newlines();
        while self.is_kind(TokenKind::Identifier) {
            self.parse_non();
        }
    }

    fn parse_non(&mut self) {
        let id = self.current_token.get_token_str_raw_value().unwrap();

        let mut non = self
            .missing
            .remove(&id)
            .unwrap_or(Rc::new(Non::from_id(id)));
        let mut_non = Rc::get_mut(&mut non).unwrap();

        self.advance();

        if !self.eat(TokenKind::Colon) {
            panic!("Colon required after non declaration.");
        }

        self.skip_spaces();

        if self.is_kind(TokenKind::Identifier) {
            let parent_name = self.current_token.get_token_str_raw_value().unwrap();
            let parent = self.find_nom_by_id_or_create(parent_name);
            mut_non.parent = Some(parent);
            self.advance();
        }

        if !self.eat(TokenKind::Newline) {
            panic!("Newline required.");
        }

        while self.eat(TokenKind::Dot) {
            let (field_name, field_value) = self.parse_field();
            mut_non.add_field(field_name, field_value);
        }

        self.noms.insert(non.id.clone(), non);

        self.skip_newlines();
    }

    fn find_nom_by_id_or_create(&mut self, id: String) -> Rc<Non> {
        if self.noms.contains_key(&id) {
            self.noms.get(&id).cloned().unwrap()
        } else {
            let non = Rc::new(Non::default());
            self.missing.insert(id.clone(), non.clone());
            non
        }
    }

    fn parse_field(&mut self) -> (String, FieldValue) {
        let field_name = if self.is_kind(TokenKind::Identifier) {
            self.current_token.get_token_str_raw_value().unwrap()
        } else {
            panic!("Field name must be an identifier.");
        };

        let mut value_vec = Vec::new();
        self.advance();
        while !self.eat(TokenKind::Newline) {
            if self.eat(TokenKind::Space) {
                let value = match self.current_token.kind {
                    TokenKind::Dot => {
                        self.advance();
                        if self.is_kind(TokenKind::Identifier) {
                            FieldValue::FieldReference(
                                self.current_token.get_token_str_raw_value().unwrap(),
                            )
                        } else {
                            panic!("Token must be an identifier.");
                        }
                    }

                    TokenKind::Identifier => {
                        let identifier = self.current_token.get_token_str_raw_value().unwrap();
                        self.advance();
                        if self.eat(TokenKind::Dot) && self.is_kind(TokenKind::Identifier) {
                            let field = self.current_token.get_token_str_raw_value();
                            FieldValue::ObjRef(
                                self.find_nom_by_id_or_create(identifier),
                                field.unwrap(),
                            )
                        } else {
                            panic!("Identifier not found for non reference.");
                        }
                    }

                    TokenKind::Litteral => {
                        FieldValue::Litteral(self.current_token.get_token_str_raw_value().unwrap())
                    }

                    TokenKind::At => FieldValue::FieldReference("id".to_string()),

                    token => panic!("Invalid token : {:?}", token),
                };
                value_vec.push(value);
                self.advance();
            }
        }

        let value = if value_vec.len() == 1 {
            value_vec.pop().unwrap()
        } else if value_vec.len() > 1 {
            FieldValue::Vec(value_vec)
        } else {
            panic!("Field value cannot be empty.");
        };

        (field_name, value)
    }

    fn skip_spaces(&mut self) {
        loop {
            if !self.eat(TokenKind::Space) {
                break;
            }
        }
    }

    fn skip_newlines(&mut self) {
        loop {
            if !self.eat(TokenKind::Newline) {
                break;
            }
        }
    }

    fn skip_spaces_and_newlines(&mut self) {
        loop {
            if !(self.eat(TokenKind::Space) || self.eat(TokenKind::Newline)) {
                break;
            }
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

    fn advance(&mut self) {
        self.current_token = self.lexer.read_next_token().unwrap_or(Token::default());
    }
}
