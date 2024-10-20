use parser::{
    ast::{Literal, Object, Property},
    parser::Parser,
    printer::Printer,
    token::{Token, TokenType},
};

mod ast;
mod client;
mod parser;

fn main() {
    let printer = Printer;

    let p1 = Property::new(
        "msg".to_string(),
        Token::new(TokenType::Colon, ":"),
        Literal::new(String::from("hello")),
    );

    let root = Object::new(
        Token::new(TokenType::String, "{"),
        vec![p1],
        Token::new(TokenType::String, "}"),
    );

    let mut parser = Parser::new(vec![
        Token::new(TokenType::LeftBrace, "{"),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a message"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a message"),
        Token::new(TokenType::RightBrace, "}"),
    ]);

    let res = parser.parse();
    dbg!(res);

    println!("Hello, World!");
}
