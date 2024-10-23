use parser::{
    parser::Parser,
    token::{Token, TokenType},
};

mod ast;
mod client;
mod parser;

enum Node {
    List(Vec<Node>),
    Object(String, Vec<Node>, String),
    Property(Box<Node>, Box<Node>),
    Literal(String),
}

fn main() {
    let mut parser = Parser::new(vec![
        Token::new(TokenType::LeftBracket, "["),
        Token::new(TokenType::True, "true"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::LeftBrace, "{"),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a message"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a new message"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Identifier, "array"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::LeftBracket, "["),
        Token::new(TokenType::String, "Array string, whooo"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::LeftBracket, "["),
        Token::new(TokenType::RightBracket, "]"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::True, "true"),
        Token::new(TokenType::RightBracket, "]"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Identifier, "obj"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::LeftBrace, "{"),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a message"),
        Token::new(TokenType::RightBrace, "}"),
        Token::new(TokenType::RightBrace, "}"),
        Token::new(TokenType::RightBracket, "]"),
    ]);

    let res = parser.parse();
    dbg!(res);

    println!("Hello, World!");
}
