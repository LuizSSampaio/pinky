use crate::{
    ast::expression::Expression,
    token::{Kind, Token},
};
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
        self.addition()
    }

    fn addition(&mut self) -> Expression {
        let mut expr = self.multiplication();

        while self.match_tokens(&[Kind::Plus, Kind::Minus]) {
            let operator = self.previous().unwrap().to_owned();
            let right = self.multiplication();
            expr = Expression::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        expr
    }

    fn multiplication(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_tokens(&[Kind::Star, Kind::Slash]) {
            let operator = self.previous().unwrap().to_owned();
            let right = self.unary();
            expr = Expression::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_tokens(&[Kind::Not, Kind::Minus, Kind::Plus]) {
            let operator = self.previous().unwrap().to_owned();
            let operand = self.unary();

            return Expression::Unary {
                operator,
                right: Box::new(operand),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        if self.match_tokens(&[Kind::Integer]) {
            return Expression::Integer(self.previous().unwrap().lexeme.parse::<i64>().unwrap());
        }
        if self.match_tokens(&[Kind::Float]) {
            return Expression::Float(self.previous().unwrap().lexeme.parse::<f64>().unwrap());
        }

        if self.match_tokens(&[Kind::LeftParen]) {
            let expr = self.expr();
            if self.match_tokens(&[Kind::RightParen]) {
                return Expression::Grouping(Box::new(expr));
            }

            panic!("ERROR: Missing ')'")
        }

        panic!("ERROR: Unknow token {:?}", self.peek())
    }

    fn match_tokens(&mut self, kinds: &[Kind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                let _ = self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, kind: &Kind) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.peek() {
            Some(token) => &token.kind == kind,
            None => false,
        }
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.kind == Kind::EndOfFile,
            None => true,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
