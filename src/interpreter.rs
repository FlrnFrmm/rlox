mod expression;
mod lexer;

use eyre::Result;

use expression::binary::Binary;
use expression::grouping::Grouping;
use expression::literal::Literal;
use expression::unary::Unary;
use expression::Evaluator;
use expression::Expression;

use crate::interpreter::expression::Evaluatable;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn execute(&self, code: &str) -> Result<()> {
        println!("Executing code from {}", code);

        // match lexer::lexical_analysis(code) {
        //     Ok((_, tokens)) => {
        //         println!("Tokens: {:?}", tokens);
        //     }
        //     Err(e) => {
        //         println!("Error: {}", e);
        //     }
        // }

        let expression = Expression::addition(
            Expression::number(1.0),
            Expression::multiplication(Expression::number(2.0), Expression::number(3.0)),
        );

        let printer = AstPrinter {};

        println!("{}", expression.evaluate(&printer));

        Ok(())
    }
}

struct AstPrinter {}

impl Evaluator<String> for AstPrinter {
    fn evaluate_literal(&self, literal: &Literal) -> String {
        match literal {
            Literal::Number(number) => number.to_string(),
            Literal::String(string) => string.to_string(),
            Literal::Boolean(boolean) => boolean.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }

    fn evaluate_unary(&self, unary: &Unary) -> String {
        match unary {
            Unary::Not(expression) => format!("(! {})", expression.evaluate(self)),
            Unary::Negate(expression) => format!("(- {})", expression.evaluate(self)),
        }
    }

    fn evaluate_binary(&self, binary: &Binary) -> String {
        match binary {
            Binary::Addition(left, right) => {
                format!("(+ {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::Subtraction(left, right) => {
                format!("(- {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::Multiplication(left, right) => {
                format!("(* {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::Division(left, right) => {
                format!("(/ {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::Equal(left, right) => {
                format!("(== {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::NotEqual(left, right) => {
                format!("(!= {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::Greater(left, right) => {
                format!("(> {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::GreaterEqual(left, right) => {
                format!("(>= {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::Less(left, right) => {
                format!("(< {} {})", left.evaluate(self), right.evaluate(self))
            }
            Binary::LessEqual(left, right) => {
                format!("(<= {} {})", left.evaluate(self), right.evaluate(self))
            }
        }
    }

    fn evaluate_grouping(&self, grouping: &Grouping) -> String {
        match grouping {
            Grouping::Expression(expression) => format!("({})", expression.evaluate(self)),
        }
    }
}
