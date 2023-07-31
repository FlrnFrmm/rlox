use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Solidus,
    Dot,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Into<Token> for Operator {
    fn into(self) -> Token {
        Token::Operator(self)
    }
}
