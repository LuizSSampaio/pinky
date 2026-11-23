use thiserror::Error;

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

    pub fn tokenize(&mut self) -> Result<&Vec<Token>, LexerError> {
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
                '-' => {
                    if self.next_is('-') {
                        while self.peek() != Some(&'\n') && !self.eof() {
                            _ = self.advance();
                        }
                    } else {
                        self.add_token(Kind::Minus);
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
                '*' => self.add_token(Kind::Star),
                '^' => self.add_token(Kind::Caret),
                '/' => self.add_token(Kind::Slash),
                ';' => self.add_token(Kind::Semicolon),
                '?' => self.add_token(Kind::Question),
                '%' => self.add_token(Kind::Mod),
                '=' => {
                    if self.next_is('=') {
                        self.add_token(Kind::Equal);
                    }
                }
                '~' => {
                    if self.next_is('~') {
                        self.add_token(Kind::NotEqual);
                    } else {
                        self.add_token(Kind::Not);
                    }
                }
                '<' => {
                    if self.next_is('=') {
                        self.add_token(Kind::LessEqual);
                    } else if self.next_is('<') {
                        self.add_token(Kind::LessLess);
                    } else {
                        self.add_token(Kind::Less);
                    }
                }
                '>' => {
                    if self.next_is('=') {
                        self.add_token(Kind::GreaterEqual);
                    } else if self.next_is('>') {
                        self.add_token(Kind::GreaterGreater);
                    } else {
                        self.add_token(Kind::Greater);
                    }
                }
                ':' => {
                    if self.next_is('=') {
                        self.add_token(Kind::Assign);
                    } else {
                        self.add_token(Kind::Colon);
                    }
                }
                '0'..='9' => self.handle_digit(),
                '\'' | '"' => self.handle_string()?,
                'a'..='z' | 'A'..='Z' | '_' => self.handle_identifier(),
                _ => {}
            }
        }

        self.add_token(Kind::EndOfFile);
        Ok(&self.tokens)
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
        match self.source.get(self.current + next - 1) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn lookback(&self, back: usize) -> Option<&char> {
        match self.source.get(self.current - back) {
            Some(ch) => Some(ch),
            None => None,
        }
    }

    fn next_is(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(char) if char == &expected => {
                _ = self.advance();
                true
            }
            _ => false,
        }
    }

    fn eof(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn handle_digit(&mut self) {
        while let Some(ch) = self.peek()
            && ch.is_ascii_digit()
        {
            self.advance();
        }

        if let Some(ch) = self.peek()
            && ch == &'.'
            && let Some(ch) = self.lookahead(2)
            && ch.is_ascii_digit()
        {
            self.advance();

            while let Some(ch) = self.peek()
                && ch.is_ascii_digit()
            {
                self.advance();
            }

            self.add_token(Kind::Float);
            return;
        }

        self.add_token(Kind::Integer);
    }

    fn handle_string(&mut self) -> Result<(), LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let delimiter = self
            .lookback(1)
            .expect("to use handle_string at least one char must be consumed")
            .to_owned();

        loop {
            match self.peek() {
                Some(&ch)
                    if ch == delimiter
                        && *self
                            .lookback(1)
                            .expect("at least the opening delimiter was consumed")
                            != '\\' =>
                {
                    break;
                }
                Some(_) => {
                    self.advance();
                }
                None => {
                    return Err(LexerError::UnterminatedString {
                        column: start_column,
                        line: start_line,
                    });
                }
            }
        }

        self.advance();
        self.add_token(Kind::String);
        Ok(())
    }

    fn handle_identifier(&mut self) {
        while let Some(ch) = self.peek()
            && (ch.is_ascii_alphanumeric() || ch == &'_')
        {
            self.advance();
        }

        self.add_token(
            Kind::get_keyword(
                &self.source[self.start..self.current]
                    .iter()
                    .collect::<String>(),
            )
            .unwrap_or(Kind::Identifier),
        );
    }

    fn add_token(&mut self, kind: Kind) {
        self.tokens.push(Token {
            kind,
            lexeme: self.source[self.start..self.current].iter().collect(),
            position: Position {
                row: self.line,
                column: if self.column > 1 {
                    self.column - 1
                } else {
                    self.column
                },
            },
        });
    }
}

#[derive(Error, Debug)]
pub enum LexerError {
    #[error(
        "unterminated string literal starting at {line}:{column}; expected closing quote before end of file"
    )]
    UnterminatedString { column: usize, line: usize },
}
