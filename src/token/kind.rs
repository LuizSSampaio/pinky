#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftSquar,
    RightSquar,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    Semicolon,
    Slash,
    Star,
    Caret,
    Mod,
    Bang,
    Question,
    Not,
    Greater,
    Less,

    // Two character tokens.
    Equal,
    GreaterEqual,
    LessEqual,
    NotEqual,
    Assign,
    GreaterGreater,
    LessLess,

    // Literals.
    Identifier,
    String,
    Integer,
    Float,

    // Keywords.
    If,
    Then,
    Else,
    True,
    False,
    And,
    Or,
    While,
    Do,
    For,
    Func,
    Null,
    End,
    Print,
    Println,

    EndOfFile,
}
