use super::Expression;

pub enum Unary {
    Not(Box<Expression>),
    Negate(Box<Expression>),
}

impl Into<Expression> for Unary {
    fn into(self) -> Expression {
        Expression::Unary(self)
    }
}
