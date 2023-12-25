pub type CursorPos = (usize, usize);

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub spelling: String,
    pub begin: CursorPos,
    pub end: CursorPos,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: String, begin: CursorPos, end: CursorPos) -> Self {
        Self {
            kind,
            spelling,
            begin,
            end,
        }
    }

    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenKind {
    // Keywords
    Extern,
    Export,
    Import,
    Let,
    Mut,
    Restrict,
    Return,
    Type,

    // Primitive types
    I8,
    I32,
    I64,

    // Separators
    Assign,
    Arrow,
    Colon,
    Comma,
    Dot,
    Ellipsis,
    Semicolon,
    To,
    Backslash,

    // Operators
    Plus,
    Minus,
    Multiply,
    Not,
    Divide,
    Equal,
    NotEqual,
    Increment,
    Decrement,

    // Brackets
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftChevron,
    RightChevron,

    // Literals
    BoolLiteral,
    CharLiteral,
    IntLiteral,
    FloatLiteral,
    StrLiteral,

    // Other
    Identifier,
    Unknown,
    EOF,
}
