use std::fmt::Display;

use super::{
    ast::{AstNode, Literal, Object, Property},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String),
}

impl std::error::Error for ParserError { }

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken(error_message) => {
                write!(f, "{}", error_message.as_str())
            }
        }
    }
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { current: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<impl AstNode, ParserError> {
        self.parse_object()
    }

    fn parse_object(&mut self) -> Result<impl AstNode, ParserError> {
        let left = self.get_current_advance().clone();

        let mut properties = vec![];

        if self.get_current_token().token_type != TokenType::RightBrace {
            properties.push(self.parse_property()?);

            while self.get_current_token().token_type == TokenType::Comma {
                self.get_current_advance();
                properties.push(self.parse_property()?);
            }
        }

        let right = self.get_current_advance().clone();

        Ok(Object::new(left, properties, right))
    }

    fn parse_property(&mut self) -> Result<Property, ParserError> {
        let key = self
            .get_current_or_error(TokenType::Identifier, "Expected identifier")?
            .clone()
            .literal;

        let colon = self.get_current_advance().clone();
        let value = self.parse_literal();

        Ok(Property::new(key, colon, value))
    }

    fn parse_literal(&mut self) -> Literal {
        Literal::new(self.get_current_advance().literal.clone())
    }

    fn get_current_or_error(
        &mut self,
        token_type: TokenType,
        error: &str,
    ) -> Result<&Token, ParserError> {
        if token_type == self.get_current_token().token_type {
            self.current += 1;
            return Ok(self.get_current_token());
        }

        Err(ParserError::UnexpectedToken(error.to_string()))
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
