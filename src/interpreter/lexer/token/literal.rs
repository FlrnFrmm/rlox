use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

impl Into<Token> for Literal {
    fn into(self) -> Token {
        Token::Literal(self)
    }
}
