use std::fmt::Debug;

use super::token::Token;

pub trait Visitor {
    fn visit_property(&self, property: &Property);
    fn visit_object(&self, object: &Object);
    fn visit_array(&self, array: &Array);
    fn visit_literal(&self, literal: &Literal);
}

pub trait AstNode: Debug {
    fn accept(&self, visitor: &dyn Visitor);
}

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub colon: Token,
    pub value: Box<dyn AstNode>,
}

impl Property {
    pub fn new(key: String, colon: Token, value: Box<dyn AstNode>) -> Self {
        Self { key, colon, value }
    }
}

impl AstNode for Property {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_property(&self);
    }
}

#[derive(Debug)]
pub struct Object {
    pub left_brace: Token,
    pub properties: Vec<Box<dyn AstNode>>,
    pub right_brace: Token,
}

impl Object {
    pub fn new(left_brace: Token, properties: Vec<Box<dyn AstNode>>, right_brace: Token) -> Self {
        Self {
            left_brace,
            properties,
            right_brace,
        }
    }
}

impl AstNode for Object {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_object(&self);
    }
}

#[derive(Debug)]
pub struct Array {
    left_bracket: Token,
    nodes: Vec<Box<dyn AstNode>>,
    right_bracket: Token,
}

impl Array {
    pub fn new(
        left_bracket: Token,
        nodes: Vec<Box<dyn AstNode>>,
        right_bracket: Token,
    ) -> Self {
        Self {
            left_bracket,
            nodes,
            right_bracket,
        }
    }
}

impl AstNode for Array {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_array(self);
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
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit_literal(self);
    }
}
