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

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    // Keywords
    Extern,
    Import,
    Let,

    // Separators
    Assign,
    Arrow,
    Comma,
    Dot,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,

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
    IntLiteral,
    FloatLiteral,
    StringLiteral,

    // Other
    Unknown,
    EOF,
}