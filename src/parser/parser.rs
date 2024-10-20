use super::{ast::{AstNode, Object, Property}, token::{Token, TokenType}};


pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current: 0,
            tokens,
        }
    }

    pub fn parse(&mut self) -> impl AstNode {
        self.parse_object()
    }

    fn parse_object(&mut self) -> impl AstNode {
        let left = self.get_current_advance().clone();

        let mut properties = vec![];

        if self.get_current_token().token_type != TokenType::RightBrace {
            properties.push(self.parse_property());
        }

        let right = self.get_current_advance().clone();

        Object::new(left, properties, right)
    }

    fn parse_property(&mut self) -> Property {
        let key = self.get_current_advance().clone().literal;
        let colon = self.get_current_advance().clone();
        let value = self.get_current_advance().clone().literal;

        Property::new(key, colon, value)
    }

    fn get_current_advance(&mut self) -> &Token {
        let token = self.tokens.get(self.current).unwrap();
        self.current += 1;
        token
    }

    fn get_current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }
}
