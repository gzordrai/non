use std::collections::HashMap;
use typed_arena::Arena;

use crate::{
    lexer::NonLexer,
    non::{FieldValue, Non},
    token::{Token, TokenKind},
};

pub struct NonParser<'a> {
    current_token: Token,
    lexer: NonLexer<'a>,
    arena: &'a Arena<Non<'a>>,
    missing: HashMap<String, *mut Non<'a>>,
    pub nons: HashMap<String, &'a Non<'a>>,
}

impl<'a> NonParser<'a> {
    pub fn new(lexer: NonLexer<'a>, arena: &'a Arena<Non<'a>>) -> Self {
        Self {
            current_token: Token::default(),
            nons: HashMap::new(),
            missing: HashMap::new(),
            arena,
            lexer,
        }
    }

    pub fn parse(&mut self) {
        self.advance();
        self.skip_newlines();

        while self.is_kind(TokenKind::Identifier) {
            self.parse_non();
        }

        if !self.missing.is_empty() {
            println!("Missing non in file:");
            for id in self.missing.keys() {
                println!("{}", id);
            }
            panic!();
        }
    }

    fn parse_non(&mut self) {
        let id = self.current_token.get_token_str_raw_value().unwrap();

        let non_ptr = if let Some(&ptr) = self.missing.get(&id) {
            self.missing.remove(&id);
            ptr
        } else {
            let non = self.arena.alloc(Non::from_id(id.clone()));
            non as *mut Non<'a>
        };

        self.advance();

        if !self.eat(TokenKind::Colon) {
            panic!("Colon required after non declaration.");
        }

        let mut parent_ids = Vec::new();
        while self.eat(TokenKind::Space) && self.is_kind(TokenKind::Identifier) {
            let parent_name = self.current_token.get_token_str_raw_value().unwrap();
            parent_ids.push(parent_name);
            self.advance();
        }

        if !self.eat(TokenKind::Newline) {
            panic!("Newline required.");
        }

        let mut fields = Vec::new();
        while self.eat(TokenKind::Dot) {
            fields.push(self.parse_field());
        }

        unsafe {
            let non = &mut *non_ptr;

            for parent_id in parent_ids {
                let parent = self.find_nom_by_id_or_create(parent_id);
                non.parents.push(parent);
            }

            for (field_name, field_value) in fields {
                non.add_field(field_name, field_value);
            }

            let id = non.id().to_string();
            self.nons.insert(id, &*non);
        }

        self.skip_newlines();
    }

    fn find_nom_by_id_or_create(&mut self, id: String) -> &'a Non<'a> {
        if let Some(&non) = self.nons.get(&id) {
            non
        } else if let Some(&ptr) = self.missing.get(&id) {
            unsafe { &*ptr }
        } else {
            let non = self.arena.alloc(Non::from_id(id.clone()));
            self.missing.insert(id, non as *mut Non<'a>);
            non
        }
    }

    fn parse_field(&mut self) -> (String, FieldValue<'a>) {
        let field_name = if self.is_kind(TokenKind::Identifier) {
            self.current_token.get_token_str_raw_value().unwrap()
        } else {
            panic!("Field name must be an identifier.");
        };

        let mut value_vec = Vec::new();
        self.advance();

        while !(self.eat(TokenKind::Newline) || self.eat(TokenKind::Eof)) {
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
                            let field = self.current_token.get_token_str_raw_value().unwrap();
                            let obj = self.find_nom_by_id_or_create(identifier);
                            FieldValue::ObjRef(obj, field)
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
        while self.eat(TokenKind::Space) {}
    }

    fn skip_newlines(&mut self) {
        while self.eat(TokenKind::Newline) {}
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
        self.current_token = self.lexer.read_next_token().unwrap_or_default();
    }
}
