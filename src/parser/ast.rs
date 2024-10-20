use std::fmt::Debug;

use super::token::Token;

pub trait Visitor {
    type T;

    fn visit_property(&self, property: &Property) -> Self::T;
    fn visit_object(&self, property: &Object) -> Self::T;
}

pub trait AstNode: Debug {
    fn accept(&self, visitor: &impl Visitor);
}

#[derive(Debug)]
pub struct Property {
    pub key: String,
    pub colon: Token,
    pub value: String,
}

impl Property {
    pub fn new(key: String, colon: Token, value: String) -> Self {
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
