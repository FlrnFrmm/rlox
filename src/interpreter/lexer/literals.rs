use super::token::literal::Literal;
use super::token::Token;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, alphanumeric1, char, digit0, digit1, one_of};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many0;
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;

pub fn r#match(code: &str) -> IResult<&str, Token> {
    alt((identifier, number, string))(code)
}

fn identifier(code: &str) -> IResult<&str, Token> {
    map(
        recognize(pair(
            alt((tag("_"), alpha1)),
            many0(alt((tag("_"), alphanumeric1))),
        )),
        |s: &str| Literal::Identifier(s.to_string()).into(),
    )(code)
}

fn number(code: &str) -> IResult<&str, Token> {
    map(
        map_res(
            alt((
                recognize(pair(char('0'), opt(pair(char('.'), digit1)))),
                recognize(tuple((
                    one_of("1234556789"),
                    digit0,
                    opt(pair(char('.'), digit1)),
                ))),
            )),
            |s: &str| s.parse::<f64>(),
        ),
        |v| Literal::Number(v).into(),
    )(code)
}

fn string(code: &str) -> IResult<&str, Token> {
    map(
        terminated(preceded(char('"'), take_till(|c| c == '"')), char('"')),
        |s: &str| Literal::String(s.to_string()).into(),
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! literal_tests {
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

    literal_tests! {
        single_letter_identifier:
            "A",
            Literal::Identifier("A".to_string()).into(),
        single_letter_that_starts_with_a_underscore_identifier:
            "_A",
            Literal::Identifier("_A".to_string()).into(),
        multi_letter_identifier:
            "AbCdEfGh",
            Literal::Identifier("AbCdEfGh".to_string()).into(),
        multi_letter_that_starts_with_a_underscore_identifier:
            "_AbcdEfgh",
            Literal::Identifier("_AbcdEfgh".to_string()).into(),
        multi_letter_identifier_with_underscores_and_numbers:
            "AbC4_dEf2_Gh42",
            Literal::Identifier("AbC4_dEf2_Gh42".to_string()).into(),
        zero:
            "0",
            Literal::Number(0.0).into(),
        zero_with_decimal:
            "0.0",
            Literal::Number(0.0).into(),
        zero_with_decimal_and_trailing_zero:
            "0.000",
            Literal::Number(0.0).into(),
        zero_with_decimal_and_trailing_number:
            "0.123456789",
            Literal::Number(0.123456789).into(),
        integer:
            "123456789",
            Literal::Number(123456789.0).into(),
        floating_point:
            "123456789.123456789",
            Literal::Number(123456789.123456789).into(),
        empty_string:
            "\"\"",
            Literal::String("".to_string()).into(),
        string:
            "\"Hello, World!\"",
            Literal::String("Hello, World!".to_string()).into(),
        string_with_newline_and_tab:
            "\"\tHello, World!\n\"",
            Literal::String("\tHello, World!\n".to_string()).into(),
    }
}
