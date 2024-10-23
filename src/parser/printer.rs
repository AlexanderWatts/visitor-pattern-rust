use super::ast::{AstNode, Visitor};

#[derive(Debug)]
pub struct Printer;

impl Printer {
    pub fn print(&self, root: &impl AstNode) {
        root.accept(self);
    }
}

impl Visitor for Printer {
    fn visit_property(&self, property: &super::ast::Property) {
        let _ = format!("{}", property.colon.literal);
    }

    fn visit_object(&self, property: &super::ast::Object) {
        let _ = format!("{}", property.left_brace.literal);
    }

    fn visit_literal(&self, literal: &super::ast::Literal) {
        let _ = format!("{}", literal.value);
    }
}
