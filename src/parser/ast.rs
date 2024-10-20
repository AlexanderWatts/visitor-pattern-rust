use std::fmt::Debug;

use super::token::Token;

pub trait Visitor {
    type T;

    fn visit_property(&self, property: &Property) -> Self::T;
    fn visit_object(&self, object: &Object) -> Self::T;
    fn visit_literal(&self, literal: &Literal) -> Self::T;
}

pub trait AstNode: Debug {
    fn accept(&self, visitor: &impl Visitor);
}

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub colon: Token,
    pub value: Literal,
}

impl Property {
    pub fn new(key: String, colon: Token, value: Literal) -> Self {
        Self { key, colon, value }
    }
}

impl AstNode for Property {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_property(&self);
    }
}

#[derive(Debug)]
pub struct Object {
    pub left_brace: Token,
    pub properties: Vec<Property>,
    pub right_brace: Token,
}

impl Object {
    pub fn new(left_brace: Token, properties: Vec<Property>, right_brace: Token) -> Self {
        Self {
            left_brace,
            properties,
            right_brace,
        }
    }
}

impl AstNode for Object {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_object(&self);
    }
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
}

impl Literal {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl AstNode for Literal {
    fn accept(&self, visitor: &impl Visitor) {
        visitor.visit_literal(self);
    }
}
