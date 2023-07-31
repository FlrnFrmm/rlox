use super::token::punctuation::Punctuation;
use super::token::Token;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::value;
use nom::IResult;

pub fn r#match(code: &str) -> IResult<&str, Token> {
    alt((
        opening_round_bracket,
        closing_round_bracket,
        opening_curly_bracket,
        closing_curly_bracket,
        comma,
        semicolon,
    ))(code)
}

fn opening_round_bracket(code: &str) -> IResult<&str, Token> {
    value(Punctuation::OpeningRoundBracket.into(), char('('))(code)
}

fn closing_round_bracket(code: &str) -> IResult<&str, Token> {
    value(Punctuation::ClosingRoundBracket.into(), char(')'))(code)
}

fn opening_curly_bracket(code: &str) -> IResult<&str, Token> {
    value(Punctuation::OpeningCurlyBracket.into(), char('{'))(code)
}

fn closing_curly_bracket(code: &str) -> IResult<&str, Token> {
    value(Punctuation::ClosingCurlyBracket.into(), char('}'))(code)
}

fn comma(code: &str) -> IResult<&str, Token> {
    value(Punctuation::Comma.into(), char(','))(code)
}

fn semicolon(code: &str) -> IResult<&str, Token> {
    value(Punctuation::Semicolon.into(), char(';'))(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! punctuation_tests {
        ($($label:ident: $input:expr, $expected_output:expr,)*) => {
            $(
                #[test]
                fn $label() {
                    match r#match($input) {
                        Ok((tail, token)) => {
                            assert_eq!(tail, "");
                            assert_eq!(token, $expected_output);
                        },
                        Err(err) => panic!("Error: {}", err.to_string())
                    }
                }
            )*
        };
    }

    punctuation_tests! {
        opening_round_bracket:
            "(",
            Punctuation::OpeningRoundBracket.into(),
        closing_round_bracket:
            ")",
            Punctuation::ClosingRoundBracket.into(),
        opening_curly_bracket:
            "{",
            Punctuation::OpeningCurlyBracket.into(),
        closing_curly_bracket:
            "}",
            Punctuation::ClosingCurlyBracket.into(),
        comma:
            ",",
            Punctuation::Comma.into(),
        semicolon:
            ";",
            Punctuation::Semicolon.into(),
    }
}
