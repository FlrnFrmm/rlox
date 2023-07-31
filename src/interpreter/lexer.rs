mod keywords;
mod literals;
mod operators;
mod punctuations;
mod token;

use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

use nom::sequence::preceded;
use token::Token;

pub fn lexical_analysis(code: &str) -> IResult<&str, Vec<Token>> {
    let (tail, (tokens, _)) = many_till(next_token, eof)(code)?;
    Ok((tail, tokens))
}

fn next_token(code: &str) -> IResult<&str, Token> {
    preceded(
        multispace0,
        alt((
            punctuations::r#match,
            operators::r#match,
            keywords::r#match,
            literals::r#match,
        )),
    )(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::keyword::Keyword::*;
    use token::literal::Literal::*;
    use token::operator::Operator::*;
    use token::punctuation::Punctuation::*;

    macro_rules! lexical_analysis_tests {
        ($($label:ident: $input:expr, $expected_output:expr,)*) => {
            $(
                #[test]
                fn $label() {
                    match lexical_analysis($input) {
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

    lexical_analysis_tests! {
        empty:
            "",
            vec![],
        statement:
            "var a = 1;",
            vec![
                Var.into(),
                Identifier("a".to_string()).into(),
                Equal.into(),
                Number(1.0).into(),
                Semicolon.into(),
            ],
    }
}
