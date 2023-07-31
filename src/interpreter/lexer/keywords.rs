use super::token::keyword::Keyword;
use super::token::Token;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::value;
use nom::sequence::terminated;
use nom::IResult;

pub fn r#match(code: &str) -> IResult<&str, Token> {
    terminated(
        alt((
            and, class, r#else, r#false, fun, r#for, r#if, nil, or, print, r#return, r_super, this,
            r#true, var, r#while,
        )),
        multispace1,
    )(code)
}

fn and(code: &str) -> IResult<&str, Token> {
    value(Keyword::And.into(), tag("and"))(code)
}

fn class(code: &str) -> IResult<&str, Token> {
    value(Keyword::Class.into(), tag("class"))(code)
}

fn r#else(code: &str) -> IResult<&str, Token> {
    value(Keyword::Else.into(), tag("else"))(code)
}

fn r#false(code: &str) -> IResult<&str, Token> {
    value(Keyword::False.into(), tag("false"))(code)
}

fn fun(code: &str) -> IResult<&str, Token> {
    value(Keyword::Fun.into(), tag("fun"))(code)
}

fn r#for(code: &str) -> IResult<&str, Token> {
    value(Keyword::For.into(), tag("for"))(code)
}

fn r#if(code: &str) -> IResult<&str, Token> {
    value(Keyword::If.into(), tag("if"))(code)
}

fn nil(code: &str) -> IResult<&str, Token> {
    value(Keyword::Nil.into(), tag("nil"))(code)
}

fn or(code: &str) -> IResult<&str, Token> {
    value(Keyword::Or.into(), tag("or"))(code)
}

fn print(code: &str) -> IResult<&str, Token> {
    value(Keyword::Print.into(), tag("print"))(code)
}

fn r#return(code: &str) -> IResult<&str, Token> {
    value(Keyword::Return.into(), tag("return"))(code)
}

fn r_super(code: &str) -> IResult<&str, Token> {
    value(Keyword::Super.into(), tag("super"))(code)
}

fn this(code: &str) -> IResult<&str, Token> {
    value(Keyword::This.into(), tag("this"))(code)
}

fn r#true(code: &str) -> IResult<&str, Token> {
    value(Keyword::True.into(), tag("true"))(code)
}

fn var(code: &str) -> IResult<&str, Token> {
    value(Keyword::Var.into(), tag("var"))(code)
}

fn r#while(code: &str) -> IResult<&str, Token> {
    value(Keyword::While.into(), tag("while"))(code)
}

mod tests {
    use super::*;

    macro_rules! keyword_tests {
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

    keyword_tests! {
        and:
            "and ",
            Keyword::And.into(),
        class:
            "class ",
            Keyword::Class.into(),
        r#else:
            "else ",
            Keyword::Else.into(),
        r#false:
            "false ",
            Keyword::False.into(),
        fun:
            "fun ",
            Keyword::Fun.into(),
        r#for:
            "for ",
            Keyword::For.into(),
        r#if:
            "if ",
            Keyword::If.into(),
        nil:
            "nil ",
            Keyword::Nil.into(),
        or:
            "or ",
            Keyword::Or.into(),
        print:
            "print ",
            Keyword::Print.into(),
        r#return:
            "return ",
            Keyword::Return.into(),
        r_super:
            "super ",
            Keyword::Super.into(),
        this:
            "this ",
            Keyword::This.into(),
        r#true:
            "true ",
            Keyword::True.into(),
        var:
            "var ",
            Keyword::Var.into(),
        r#while:
            "while ",
            Keyword::While.into(),
    }
}
