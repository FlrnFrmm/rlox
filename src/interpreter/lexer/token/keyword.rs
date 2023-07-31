use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

impl Into<Token> for Keyword {
    fn into(self) -> Token {
        Token::Keyword(self)
    }
}
