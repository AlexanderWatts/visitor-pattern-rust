use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,

    Identifier,
    String,
    Number,
    Null,
    True,
    False,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: Literal,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Literal) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

pub trait Visitor<T> {
    fn visit_primary(&self, value: &Literal) -> T;
    fn visit_object(&self, left: &Token, properties: &Vec<Node>, right: &Token) -> T;
    fn visit_property(&self, key: &Token, colon: &Token, value: &Node) -> T;
    fn visit_list(&self, left: &Token, nodes: &Vec<Node>, right: &Token) -> T;
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::String(value) => value.to_string(),
            Literal::Number(value) => value.to_string(),
            Literal::Bool(value) => value.to_string(),
            Literal::Null => "null".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Primary(Literal),
    Object(Token, Vec<Node>, Token),
    Property(Token, Token, Box<Node>),
    List(Token, Vec<Node>, Token),
}

impl Node {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> T {
        match self {
            Self::Primary(value) => visitor.visit_primary(value),
            Self::Object(left, properties, right) => visitor.visit_object(left, properties, right),
            Self::Property(key, colon, value) => visitor.visit_property(key, colon, value),
            Self::List(left, nodes, right) => visitor.visit_list(left, nodes, right),
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

    pub fn parse(&mut self) -> Result<Node, String> {
        self.parse_literal()
    }

    fn parse_object(&mut self) -> Result<Node, String> {
        let left = self
            .get_or_error(TokenType::LeftBrace, "Expected {")?
            .clone();

        let mut p: BTreeMap<String, Node> = BTreeMap::new();

        if self.get_current_token().token_type != TokenType::RightBrace {
            match self.parse_property()? {
                Node::Property(key, colon, value) => {
                    match p.entry(key.literal.to_string()) {
                        std::collections::btree_map::Entry::Occupied(_) => Err(format!(
                            "Duplicate property key {} found",
                            key.literal.to_string()
                        ))?,
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(Node::Property(key, colon, value))
                        }
                    };
                }
                _ => {}
            }

            while self.match_token(TokenType::Comma) {
                self.get_token_advance();

                match self.parse_property()? {
                    Node::Property(key, colon, value) => {
                        match p.entry(key.literal.to_string()) {
                            std::collections::btree_map::Entry::Occupied(_) => Err(format!(
                                "Duplicate property key {} found",
                                key.literal.to_string()
                            ))?,
                            std::collections::btree_map::Entry::Vacant(entry) => {
                                entry.insert(Node::Property(key, colon, value))
                            }
                        };
                    }
                    _ => {}
                }
            }
        }

        let right = self
            .get_or_error(TokenType::RightBrace, "Expected }")?
            .clone();

        let r: Vec<Node> = p.into_values().collect();
        Ok(Node::Object(left, r, right))
    }

    fn parse_property(&mut self) -> Result<Node, String> {
        let key = self
            .get_or_error(TokenType::Identifier, "Expected identifier")?
            .clone();
        let colon = self
            .get_or_error(TokenType::Colon, "Expected colon")?
            .clone();
        let value = self.parse_literal()?;

        Ok(Node::Property(key, colon, Box::new(value)))
    }

    fn parse_list(&mut self) -> Result<Node, String> {
        let left = self
            .get_or_error(TokenType::LeftBracket, "Expected [")?
            .clone();

        let mut properties = vec![];

        if self.get_current_token().token_type != TokenType::RightBracket {
            properties.push(self.parse_literal()?);

            while self.match_token(TokenType::Comma) {
                self.get_token_advance();
                properties.push(self.parse_literal()?);
            }
        }

        let right = self
            .get_or_error(TokenType::RightBracket, "Expected ]")?
            .clone();

        Ok(Node::List(left, properties, right))
    }

    fn parse_literal(&mut self) -> Result<Node, String> {
        if self.match_token(TokenType::String) {
            return Ok(Node::Primary(self.get_token_advance().clone().literal));
        }

        if self.match_token(TokenType::Number) {
            return Ok(Node::Primary(self.get_token_advance().clone().literal));
        }

        if self.match_token(TokenType::True) {
            return Ok(Node::Primary(self.get_token_advance().clone().literal));
        }

        if self.match_token(TokenType::False) {
            return Ok(Node::Primary(self.get_token_advance().clone().literal));
        }

        if self.match_token(TokenType::Null) {
            return Ok(Node::Primary(self.get_token_advance().clone().literal));
        }

        if self.match_token(TokenType::LeftBracket) {
            return self.parse_list();
        }

        if self.match_token(TokenType::LeftBrace) {
            return self.parse_object();
        }

        Err("Unknown literal".to_string())
    }

    fn get_or_error(&mut self, token_type: TokenType, error: &str) -> Result<&Token, String> {
        if self.match_token(token_type) {
            return Ok(self.get_token_advance());
        }

        Err(error.to_string())
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.get_current_token().token_type == token_type {
            return true;
        }

        false
    }

    fn get_token_advance(&mut self) -> &Token {
        let token = self.tokens.get(self.current).unwrap();
        self.current += 1;
        token
    }

    fn get_current_token(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

struct PrettyPrint;

impl PrettyPrint {
    pub fn print(&self, root: &Node) -> String {
        root.accept(self)
    }

    pub fn dfs(&self, root: &Node) -> String {
       self.trav(root, 0)
    }

    fn trav(&self, node: &Node, mut depth: u32) -> String {
        match node {
            Node::Primary(literal) => {
                return literal.to_string()
            },
            Node::Object(left_brace, vec, right_brace) => {
                let brace_space = " ".repeat((depth * 4)as usize);

                depth += 1;

                let property_space = " ".repeat((depth * 4)as usize);

                let mut buf = String::new();

                if depth < 1 {
                    buf += "\n";
                }

                buf += left_brace.literal.to_string().as_str();

                if !vec.is_empty() {
                    buf += "\n";
                }

                for (i, node) in vec.into_iter().enumerate() {
                    buf += &property_space;
                    buf += self.trav(node, depth).as_str();

                    if i < vec.len() - 1 {
                        buf += ",";
                    }

                    buf += "\n";
                }

                if !vec.is_empty() {
                    buf += &brace_space;
                }
                buf += right_brace.literal.to_string().as_str();

                depth -= 1;

                return buf
            },
            Node::Property(key, colon, value) => {
                let mut buf = String::new();
                buf += format!("\"{}\"", &key.literal.to_string()).as_str();
                buf += &colon.literal.to_string();
                buf += " ";
                buf += &self.trav(&value, depth);

                return buf
            }
            Node::List(left_bracket, vec, right_bracket) => {
                let bracket_space = " ".repeat((depth * 4)as usize);
                depth += 1;
                let primary_space = " ".repeat((depth * 4)as usize);

                let mut buf = String::new();

                if depth < 1 {
                    buf += "\n";
                }

                buf += left_bracket.literal.to_string().as_str();

                if !vec.is_empty() {
                    buf += "\n";
                }

                for (i, node) in vec.into_iter().enumerate() {
                    buf += &primary_space;
                    buf += self.trav(node, depth).as_str();

                    if i < vec.len() - 1 {
                        buf += ",";
                    }

                    buf += "\n";
                }

                if !vec.is_empty() {
                    buf += &bracket_space;
                }
                buf += right_bracket.literal.to_string().as_str();

                depth -= 1;

                return buf
            },
        }
    }
}

impl Visitor<String> for PrettyPrint {
    fn visit_primary(&self, value: &Literal) -> String {
        if let Literal::String(value) = value {
            return format!("\"{}\"", value);
        }

        value.to_string()
    }

    fn visit_object(&self, left: &Token, properties: &Vec<Node>, right: &Token) -> String {
        let properties = properties
            .into_iter()
            .map(|node| node.accept(self))
            .collect::<String>();

        format!(
            "{}{}{}",
            left.literal.to_string(),
            properties,
            right.literal.to_string()
        )
    }

    fn visit_property(&self, key: &Token, colon: &Token, value: &Node) -> String {
        format!(
            "{}{}{}",
            key.literal.to_string(),
            colon.literal.to_string(),
            value.accept(self)
        )
    }

    fn visit_list(&self, left: &Token, nodes: &Vec<Node>, right: &Token) -> String {
        let nodes = nodes
            .into_iter()
            .map(|node| node.accept(self))
            .collect::<String>();

        format!(
            "{}{}{}",
            left.literal.to_string(),
            nodes,
            right.literal.to_string()
        )
    }
}

#[cfg(test)]
mod node_tests {
    use super::*;

    #[test]
    fn creation() {
        let pp = PrettyPrint;

        let root = Node::Object(
            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),
            vec![
                Node::Property(
                    Token::new(TokenType::Identifier, Literal::String("number".to_string())),
                    Token::new(TokenType::Colon, Literal::String(":".to_string())),
                    Box::new(Node::Primary(Literal::Number(32.0))),
                ),
                Node::Property(
                    Token::new(TokenType::Identifier, Literal::String("list".to_string())),
                    Token::new(TokenType::Colon, Literal::String(":".to_string())),
                    Box::new(Node::List(
                        Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),
                        vec![Node::Primary(Literal::Bool(false))],
                        Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
                    )),
                ),
            ],
            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),
        );

        let res = pp.print(&root).to_string();

        println!("{}", res);
    }

    #[test]
    fn parse() {
        let mut parser = Parser::new(vec![
            Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),
            Token::new(TokenType::String, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),
            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("message".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::String, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("obj".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),
            Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),
            Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
        ]);

        //let mut parser = Parser::new(vec![Token::new(TokenType::String, Literal::Number(325.0))]);
        let ast = parser.parse();

        println!("{:#?}", ast);

        let p = PrettyPrint;
        let res = p.print(&ast.unwrap());
        println!("{}", res);
    }

    #[test]
    fn dfs() {
        let mut parser = Parser::new(vec![
            Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),
            Token::new(TokenType::String, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),
            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("message".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::String, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("obj".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),
            Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),
            Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
        ]);

        let mut parser = Parser::new(vec![
            Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),

            Token::new(TokenType::True, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),


            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("obj".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::LeftBracket, Literal::String("[".to_string())),

            Token::new(TokenType::True, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),

            Token::new(TokenType::True, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),


            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("obj".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::True, Literal::Bool(true)),

            Token::new(TokenType::Comma, Literal::String(",".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("main".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::True, Literal::Bool(true)),

            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),
            
            Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),

            Token::new(TokenType::Comma, Literal::String(",".to_string())),
            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),
            Token::new(
                TokenType::Identifier,
                Literal::String("other".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::True, Literal::Bool(true)),
            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),

            Token::new(TokenType::RightBracket, Literal::String("]".to_string())),
        ]);
        let mut parser = Parser::new(vec![
            Token::new(TokenType::LeftBrace, Literal::String("{".to_string())),

            Token::new(
                TokenType::Identifier,
                Literal::String("other".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::True, Literal::Bool(true)),
            Token::new(TokenType::Comma, Literal::String(",".to_string())),

            Token::new(
                TokenType::Identifier,
                Literal::String("name".to_string()),
            ),
            Token::new(TokenType::Colon, Literal::String(":".to_string())),
            Token::new(TokenType::String, Literal::String("Alex".to_string())),

            Token::new(TokenType::RightBrace, Literal::String("}".to_string())),
        ]);
        let root = parser.parse().unwrap();
        println!("{:#?}", &root);

        let pretty = PrettyPrint;
        let res = pretty.dfs(&root);
        println!("{res}");
    }
}
