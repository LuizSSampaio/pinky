use crate::token::{Kind, Position, Token};

#[derive(Debug, Default)]
pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,

    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            ..Default::default()
        }
    }

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while self.current < self.source.len() {
            self.start = self.current;

            let ch = match self.advance() {
                Some(ch) => ch,
                None => break,
            };

            match ch {
                '+' => self.add_token(Kind::Plus),
                '-' => self.add_token(Kind::Minus),
                '*' => self.add_token(Kind::Star),
                _ => continue,
            }
        }

        &self.tokens
    }

    fn advance(&mut self) -> Option<&char> {
        match self.source.get(self.current) {
            Some(ch) => {
                self.current += 1;
                Some(ch)
            }
            None => None,
        }
    }

    fn add_token(&mut self, kind: Kind) {
        self.tokens.push(Token {
            kind,
            lexeme: self.source[self.start..self.current].iter().collect(),
            position: Position {
                row: self.line,
                column: self.current,
            },
        });
    }
}
