use super::token::operator::Operator;
use super::token::Token;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::value;
use nom::IResult;

pub fn r#match(code: &str) -> IResult<&str, Token> {
    alt((
        bang_equal,
        equal_equal,
        greater_equal,
        less_equal,
        plus,
        minus,
        star,
        solidus,
        dot,
        bang,
        equal,
        greater,
        less,
    ))(code)
}

fn plus(code: &str) -> IResult<&str, Token> {
    value(Operator::Plus.into(), char('+'))(code)
}

fn minus(code: &str) -> IResult<&str, Token> {
    value(Operator::Minus.into(), char('-'))(code)
}

fn star(code: &str) -> IResult<&str, Token> {
    value(Operator::Star.into(), char('*'))(code)
}

fn solidus(code: &str) -> IResult<&str, Token> {
    value(Operator::Solidus.into(), char('/'))(code)
}

fn dot(code: &str) -> IResult<&str, Token> {
    value(Operator::Dot.into(), char('.'))(code)
}

fn bang(code: &str) -> IResult<&str, Token> {
    value(Operator::Bang.into(), char('!'))(code)
}

fn bang_equal(code: &str) -> IResult<&str, Token> {
    value(Operator::BangEqual.into(), tag("!="))(code)
}

fn equal(code: &str) -> IResult<&str, Token> {
    value(Operator::Equal.into(), char('='))(code)
}

fn equal_equal(code: &str) -> IResult<&str, Token> {
    value(Operator::EqualEqual.into(), tag("=="))(code)
}

fn greater(code: &str) -> IResult<&str, Token> {
    value(Operator::Greater.into(), char('>'))(code)
}

fn greater_equal(code: &str) -> IResult<&str, Token> {
    value(Operator::GreaterEqual.into(), tag(">="))(code)
}

fn less(code: &str) -> IResult<&str, Token> {
    value(Operator::Less.into(), char('<'))(code)
}

fn less_equal(code: &str) -> IResult<&str, Token> {
    value(Operator::LessEqual.into(), tag("<="))(code)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! operator_tests {
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

    operator_tests! {
        plus:
            "+",
            Operator::Plus.into(),
        minus:
            "-",
            Operator::Minus.into(),
        star:
            "*",
            Operator::Star.into(),
        solidus:
            "/",
            Operator::Solidus.into(),
        dot:
            ".",
            Operator::Dot.into(),
        bang:
            "!",
            Operator::Bang.into(),
        bang_equal:
            "!=",
            Operator::BangEqual.into(),
        equal:
            "=",
            Operator::Equal.into(),
        equal_equal:
            "==",
            Operator::EqualEqual.into(),
        greater:
            ">",
            Operator::Greater.into(),
        greater_equal:
            ">=",
            Operator::GreaterEqual.into(),
        less:
            "<",
            Operator::Less.into(),
        less_equal:
            "<=",
            Operator::LessEqual.into(),
    }
}
