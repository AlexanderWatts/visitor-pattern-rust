use parser::{
    parser::Parser,
    token::{Token, TokenType},
};

mod ast;
mod client;
mod parser;

fn main() {
    let mut parser = Parser::new(vec![
        Token::new(TokenType::LeftBrace, "{"),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a message"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Identifier, "msg"),
        Token::new(TokenType::Colon, ":"),
        Token::new(TokenType::String, "This is a new message"),
        Token::new(TokenType::RightBrace, "}"),
    ]);

    let res = parser.parse();
    dbg!(res);

    println!("Hello, World!");
}
