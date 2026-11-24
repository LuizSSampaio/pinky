use thiserror::Error;

use crate::{
    ast::expression::Expression,
    parser::ParserError::{MissingToken, UnknowToken},
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

    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        self.expr()
    }

    fn expr(&mut self) -> Result<Expression, ParserError> {
        self.addition()
    }

    fn addition(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.multiplication()?;

        while self.match_tokens(&[Kind::Plus, Kind::Minus]) {
            let operator = self
                .previous()
                .expect("Previous token must exist after a success match_tokens")
                .to_owned();
            let right = self.multiplication()?;
            expr = Expression::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[Kind::Star, Kind::Slash]) {
            let operator = self
                .previous()
                .expect("Previous token must exist after a success match_tokens")
                .to_owned();
            let right = self.unary()?;
            expr = Expression::Binary {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_tokens(&[Kind::Not, Kind::Minus, Kind::Plus]) {
            let operator = self
                .previous()
                .expect("Previous token must exist after a success match_tokens")
                .to_owned();
            let operand = self.unary()?;

            return Ok(Expression::Unary {
                operator,
                right: Box::new(operand),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        if self.match_tokens(&[Kind::Integer]) {
            return Ok(Expression::Integer(
                self.previous()
                    .expect("Previous token must exist after a success match_tokens")
                    .lexeme
                    .parse::<i64>()
                    .expect("Integer token lexeme must always be parsed with success"),
            ));
        }
        if self.match_tokens(&[Kind::Float]) {
            return Ok(Expression::Float(
                self.previous()
                    .expect("Previous token must exist after a success match_tokens")
                    .lexeme
                    .parse::<f64>()
                    .expect("Float token lexeme must always be parsed with success"),
            ));
        }

        if self.match_tokens(&[Kind::LeftParen]) {
            let expr = self.expr()?;
            if self.match_tokens(&[Kind::RightParen]) {
                return Ok(Expression::Grouping(Box::new(expr)));
            }

            return Err(MissingToken(Kind::RightParen));
        }

        Err(UnknowToken(
            self.peek()
                .expect("Unknow token must at least exist")
                .to_owned(),
        ))
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

#[derive(Error, Debug)]
pub enum ParserError {
    #[error(
        "unexpected token '{}' ({:?}) at {}:{}",
        .0.lexeme,
        .0.kind,
        .0.position.row,
        .0.position.column
    )]
    UnknowToken(Token),
    #[error("missing expected token {0:?}")]
    MissingToken(Kind),
}
