use super::Expression;

pub enum Grouping {
    Expression(Box<Expression>),
}

impl Into<Expression> for Grouping {
    fn into(self) -> Expression {
        Expression::Grouping(self)
    }
}
