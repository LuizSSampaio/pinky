use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Binary {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Grouping(Box<Expression>),
}
