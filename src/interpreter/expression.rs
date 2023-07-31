pub mod binary;
pub mod grouping;
pub mod literal;
pub mod unary;

use binary::Binary;
use grouping::Grouping;
use literal::Literal;
use unary::Unary;

pub enum Expression {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

impl Expression {
    pub fn number(number: f64) -> Self {
        Literal::Number(number).into()
    }

    pub fn string(string: String) -> Self {
        Literal::String(string).into()
    }

    pub fn boolean(boolean: bool) -> Self {
        Literal::Boolean(boolean).into()
    }

    pub fn nil() -> Self {
        Literal::Nil.into()
    }

    pub fn not(expression: Self) -> Self {
        Unary::Not(Box::new(expression)).into()
    }

    pub fn negate(expression: Self) -> Self {
        Unary::Negate(Box::new(expression)).into()
    }

    pub fn addition(left_expression: Self, right_expression: Self) -> Self {
        Binary::Addition(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn subtraction(left_expression: Self, right_expression: Self) -> Self {
        Binary::Subtraction(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn multiplication(left_expression: Self, right_expression: Self) -> Self {
        Binary::Multiplication(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn division(left_expression: Self, right_expression: Self) -> Self {
        Binary::Division(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn equal(left_expression: Self, right_expression: Self) -> Self {
        Binary::Equal(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn not_equal(left_expression: Self, right_expression: Self) -> Self {
        Binary::NotEqual(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn greater(left_expression: Self, right_expression: Self) -> Self {
        Binary::Greater(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn greater_equal(left_expression: Self, right_expression: Self) -> Self {
        Binary::GreaterEqual(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn less(left_expression: Self, right_expression: Self) -> Self {
        Binary::Less(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn less_equal(left_expression: Self, right_expression: Self) -> Self {
        Binary::LessEqual(Box::new(left_expression), Box::new(right_expression)).into()
    }

    pub fn grouping(expression: Self) -> Self {
        Grouping::Expression(Box::new(expression)).into()
    }
}

impl<T> Evaluatable<T> for Expression {
    fn evaluate(&self, evaluator: &dyn Evaluator<T>) -> T {
        match self {
            Expression::Literal(literal) => evaluator.evaluate_literal(literal),
            Expression::Unary(unary) => evaluator.evaluate_unary(unary),
            Expression::Binary(binary) => evaluator.evaluate_binary(binary),
            Expression::Grouping(grouping) => evaluator.evaluate_grouping(grouping),
        }
    }
}

pub trait Evaluatable<T> {
    fn evaluate(&self, evaluator: &dyn Evaluator<T>) -> T;
}

pub trait Evaluator<T> {
    fn evaluate_literal(&self, literal: &Literal) -> T;
    fn evaluate_unary(&self, unary: &Unary) -> T;
    fn evaluate_binary(&self, binary: &Binary) -> T;
    fn evaluate_grouping(&self, grouping: &Grouping) -> T;
}
