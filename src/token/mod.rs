mod kind;

pub use kind::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: Kind,
    pub lexeme: String,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}
