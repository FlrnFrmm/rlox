use super::Expression;

pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl Into<Expression> for Literal {
    fn into(self) -> Expression {
        Expression::Literal(self)
    }
}
