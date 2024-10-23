use std::fmt::Display;

use super::{
    ast::{Array, AstNode, Literal, Object, Property},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(String),
}

impl std::error::Error for ParserError {}

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

    pub fn parse(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        if self.get_current_token().token_type == TokenType::LeftBrace {
            return self.parse_object();
        }

        self.parse_array()
    }

    fn parse_object(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        let left = self
            .get_current_or_error(TokenType::LeftBrace, "Expected {")?
            .clone();

        let mut properties = vec![];

        if self.get_current_token().token_type != TokenType::RightBrace {
            properties.push(self.parse_property()?);

            while self.get_current_token().token_type == TokenType::Comma {
                self.get_current_advance();
                properties.push(self.parse_property()?);
            }
        }

        let right = self
            .get_current_or_error(TokenType::RightBrace, "Expected }")?
            .clone();

        dbg!(&right);

        Ok(Box::new(Object::new(left, properties, right)))
    }

    fn parse_property(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        let key = self
            .get_current_or_error(TokenType::Identifier, "Expected identifier")?
            .clone()
            .literal;

        let colon = self
            .get_current_or_error(TokenType::Colon, "Expected :")?
            .clone();

        let value = self.parse_literal()?;

        Ok(Box::new(Property::new(key, colon, value)))
    }

    fn parse_array(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        let left_bracket = self
            .get_current_or_error(TokenType::LeftBracket, "Expected [")?
            .clone();

        let mut nodes = vec![];

        dbg!(self.get_current_token());
        if self.get_current_token().token_type != TokenType::RightBracket {
            dbg!(self.get_current_token());
            nodes.push(self.parse_literal()?);

            while self.get_current_token().token_type == TokenType::Comma {
                self.get_current_advance();
                nodes.push(self.parse_literal()?);
            }
        }

        let right_bracket = self
            .get_current_or_error(TokenType::RightBracket, "Expected ]")?
            .clone();

        Ok(Box::new(Array::new(left_bracket, nodes, right_bracket)))
    }

    fn parse_literal(&mut self) -> Result<Box<dyn AstNode>, ParserError> {
        if self.get_current_token().token_type == TokenType::LeftBrace {
            return self.parse_object();
        }

        if self.get_current_token().token_type == TokenType::LeftBracket {
            return self.parse_array();
        }

        if self.get_current_token().token_type == TokenType::String {
            return Ok(Box::new(Literal::new(
                self.get_current_advance().literal.clone(),
            )));
        }

        if self.get_current_token().token_type == TokenType::Number {
            return Ok(Box::new(Literal::new(
                self.get_current_advance().literal.clone(),
            )));
        }

        if self.get_current_token().token_type == TokenType::True {
            return Ok(Box::new(Literal::new(
                self.get_current_advance().literal.clone(),
            )));
        }

        if self.get_current_token().token_type == TokenType::False {
            return Ok(Box::new(Literal::new(
                self.get_current_advance().literal.clone(),
            )));
        }

        Err(ParserError::UnexpectedToken("Unknown literal".to_string()))
    }

    fn get_current_or_error(
        &mut self,
        token_type: TokenType,
        error: &str,
    ) -> Result<&Token, ParserError> {
        if token_type == self.get_current_token().token_type {
            return Ok(self.get_current_advance());
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
