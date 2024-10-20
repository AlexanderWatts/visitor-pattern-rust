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
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Self {
            token_type,
            literal: String::from(literal),
        }
    }
}
