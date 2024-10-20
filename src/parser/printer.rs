use super::ast::{AstNode, Visitor};

#[derive(Debug)]
pub struct Printer;

impl Printer {
    pub fn print(&self, root: &impl AstNode) {
        root.accept(self);
    }
}

impl Visitor for Printer {
    type T = String;

    fn visit_property(&self, property: &super::ast::Property) -> Self::T {
        format!("{}", property.colon.literal)
    }

    fn visit_object(&self, property: &super::ast::Object) -> Self::T {
        format!("{}", property.left_brace.literal)
    }
}
