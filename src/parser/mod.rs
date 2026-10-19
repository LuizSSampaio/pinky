use crate::{ast::expression::Expression, token::Token};
#[derive(Debug, Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    pub fn parse(&mut self) -> Expression {
        self.expr()
    }

    fn expr(&mut self) -> Expression {
        self.term();
        todo!()
    }

    fn term(&mut self) -> Expression {
        self.factor();
        todo!()
    }

    fn factor(&mut self) -> Expression {
        self.unary();
        todo!()
    }

    fn unary(&mut self) -> Expression {
        self.primary();
        todo!()
    }

    fn primary(&mut self) -> Expression {
        todo!()
    }
}
