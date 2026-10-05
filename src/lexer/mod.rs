use crate::token::{Kind, Position, Token};

#[derive(Debug, Default)]
pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,

    column: usize,
    line: usize,

    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
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
                '\n' => {
                    self.column = 0;
                    self.line += 1;
                }
                '#' => {
                    while self.peek() != Some(&'\n') {
                        self.advance();
                    }
                }
                '(' => self.add_token(Kind::LeftParen),
                ')' => self.add_token(Kind::RightParen),
                '{' => self.add_token(Kind::LeftCurly),
                '}' => self.add_token(Kind::RightCurly),
                '[' => self.add_token(Kind::LeftSquar),
                ']' => self.add_token(Kind::RightSquar),
                '.' => self.add_token(Kind::Dot),
                ',' => self.add_token(Kind::Comma),
                '+' => self.add_token(Kind::Plus),
                '-' => self.add_token(Kind::Minus),
                '*' => self.add_token(Kind::Star),
                '^' => self.add_token(Kind::Caret),
                '/' => self.add_token(Kind::Slash),
                ';' => self.add_token(Kind::Semicolon),
                '?' => self.add_token(Kind::Question),
                '%' => self.add_token(Kind::Mod),
                _ => {}
            }
        }

        &self.tokens
    }

    fn advance(&mut self) -> Option<&char> {
        match self.source.get(self.current) {
            Some(ch) => {
                self.current += 1;
                self.column += 1;
                Some(ch)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&char> {
        self.lookahead(1)
    }

    fn lookahead(&self, next: usize) -> Option<&char> {
        match self.source.get(self.current + next) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn next_is(&mut self, expected: char) -> bool {
        if self.current >= self.source.len() {
            return false;
        }

        match self.peek() {
            Some(char) if char == &expected => {
                _ = self.advance();
                true
            }
            _ => false,
        }
    }

    fn add_token(&mut self, kind: Kind) {
        self.tokens.push(Token {
            kind,
            lexeme: self.source[self.start..self.current].iter().collect(),
            position: Position {
                row: self.line,
                column: self.column - 1,
            },
        });
    }
}
