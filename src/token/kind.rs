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
    Ret,

    EndOfFile,
}

impl Kind {
    pub fn get_keyword(text: &str) -> Option<Kind> {
        match text {
            "if" => Some(Kind::If),
            "else" => Some(Kind::Else),
            "then" => Some(Kind::Then),
            "true" => Some(Kind::True),
            "false" => Some(Kind::False),
            "and" => Some(Kind::And),
            "or" => Some(Kind::Or),
            "while" => Some(Kind::While),
            "do" => Some(Kind::Do),
            "for" => Some(Kind::For),
            "func" => Some(Kind::Func),
            "null" => Some(Kind::Null),
            "end" => Some(Kind::End),
            "print" => Some(Kind::Print),
            "println" => Some(Kind::Println),
            "ret" => Some(Kind::Ret),
            _ => None,
        }
    }
}
