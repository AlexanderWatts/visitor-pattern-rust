use super::token::Token;

pub trait Visitor<T> {
    fn visit_literal(&self, value: &String) -> T;
    fn visit_object(&self, left: &Token, properties: &Vec<Node>, right: &Token) -> T;
    fn visit_property(&self, key: &Node, colon: &Token, value: &Node) -> T;
    fn visit_list(&self, left: &Token, nodes: &Vec<Node>, right: &Token) -> T;
}

#[derive(Debug)]
pub enum Node {
    Literal(String),
    Object(Token, Vec<Node>, Token),
    Property(Box<Node>, Token, Box<Node>),
    List(Token, Vec<Node>, Token),
}

impl Node {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Self::Literal(value) => visitor.visit_literal(value),
            Self::Object(left, properties, right) => visitor.visit_object(left, properties, right),
            Self::Property(key, colon, value) => visitor.visit_property(key, colon, value),
            Self::List(left, nodes, right) => visitor.visit_list(left, nodes, right),
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
    fn visit_literal(&self, value: &String) -> String {
        value.to_string()
    }

    fn visit_object(&self, left: &Token, properties: &Vec<Node>, right: &Token) -> String {
        let properties = properties
            .into_iter()
            .map(|node| node.accept(self))
            .collect::<String>();

        format!("{}{}{}", left.literal, properties, right.literal)
    }

    fn visit_property(&self, key: &Node, colon: &Token, value: &Node) -> String {
        format!(
            "{}{}{}",
            key.accept(self),
            colon.literal,
            value.accept(self)
        )
    }

    fn visit_list(&self, left: &Token, nodes: &Vec<Node>, right: &Token) -> String {
        let nodes = nodes
            .into_iter()
            .map(|node| node.accept(self))
            .collect::<String>();

        format!("{}{}{}", left.literal, nodes, right.literal)
    }
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
                Node::Property(
                    Box::new(Node::Literal("message".to_string())),
                    Token::new(TokenType::Colon, ":"),
                    Box::new(Node::Literal("23".to_string())),
                ),
                Node::Property(
                    Box::new(Node::Literal("data".to_string())),
                    Token::new(TokenType::Colon, ":"),
                    Box::new(Node::List(
                        Token::new(TokenType::LeftBracket, "["),
                        vec![
                            Node::Literal("false".to_string()),
                        ],
                        Token::new(TokenType::RightBracket, "]"),
                    )),
                ),
            ],
            Token::new(TokenType::RightBrace, "}"),
        );

        let res = pp.print(&root).to_string();

        print!("{}", res);
    }
}
