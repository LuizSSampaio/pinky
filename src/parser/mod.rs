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
