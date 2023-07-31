pub mod keyword;
pub mod literal;
pub mod operator;
pub mod punctuation;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operator(operator::Operator),
    Punctuation(punctuation::Punctuation),
    Keyword(keyword::Keyword),
    Literal(literal::Literal),
}
