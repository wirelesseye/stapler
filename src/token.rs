pub type CursorPos = (usize, usize);

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    spelling: String,
    begin: CursorPos,
    end: CursorPos,
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

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenKind {
    // Keywords
    Extern,
    Import,
    Let,
    Mut,

    // Separators
    Assign,
    Arrow,
    Comma,
    Dot,
    Colon,
    Semicolon,
    To,

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
    StringLiteral,

    // Other
    Identifier,
    Unknown,
    EOF,
}