use super::token::Token;

pub enum Node {
    Literal(String),
    Object(Token, Vec<Node>, Token),
    Property(Box<Node>, Token, Box<Node>),
    List(Token, Vec<Node>, Token),
}

impl Node {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Self::Literal(_) => visitor.visit_literal(self),
            Self::Object(_, _, _) => visitor.visit_object(self),
            Self::Property(_, _, _) => visitor.visit_property(self),
            Self::List(_, _, _) => visitor.visit_list(self),
        }
    }
}

struct PrettyPrint;

impl PrettyPrint {
    pub fn print(&self, root: &Node) -> String {
        root.accept(self)  
    }
}

impl Visitor<String> for PrettyPrint {
    fn visit_literal(&self, literal: &Node) -> String {
        "literal".to_string()
    }

    fn visit_object(&self, object: &Node) -> String {
        let i = "object".to_string();


        i
    }

    fn visit_property(&self, property: &Node) -> String {
        "property".to_string()
    }

    fn visit_list(&self, list: &Node) -> String {
        "list".to_string()
    }
}

pub trait Visitor<T> {
    fn visit_literal(&self, literal: &Node) -> T;
    fn visit_object(&self, object: &Node) -> T;
    fn visit_property(&self, property: &Node) -> T;
    fn visit_list(&self, list: &Node) -> T;
}

#[cfg(test)]
mod node_tests {
    use crate::parser::token::TokenType;

    use super::*;

    #[test]
    fn creation() {
        let pp = PrettyPrint;

        let root = Node::Object(
            Token::new(TokenType::LeftBrace, "{"),
            vec![
                Node::Literal("null".to_string()),
            ],
            Token::new(TokenType::RightBrace, "}"),
        );

        let res = pp.print(&root);
        dbg!(res);
    }
}
