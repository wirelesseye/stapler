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

    pub fn is_kind(&self, kind: TokenKind) -> bool {
        self.kind() == kind
    }
    
    pub fn spelling(&self) -> &str {
        &self.spelling
    }

    pub fn begin(&self) -> CursorPos {
        self.begin
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
    Return,

    // Primitive types
    I8,
    I32,
    I64,

    // Separators
    Assign,
    Arrow,
    Comma,
    Dot,
    Colon,
    Semicolon,
    To,
    Ellipsis,

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
