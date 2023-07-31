use super::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuation {
    OpeningRoundBracket,
    ClosingRoundBracket,
    OpeningCurlyBracket,
    ClosingCurlyBracket,
    Comma,
    Semicolon,
}

impl Into<Token> for Punctuation {
    fn into(self) -> Token {
        Token::Punctuation(self)
    }
}
