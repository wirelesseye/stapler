use phf::phf_set;
use std::{fs::File, mem};

use crate::char_reader::{CharReader, EOL};
use crate::token::{CursorPos, Token, TokenKind};

const PUNCTUATIONS: phf::Set<char> = phf_set! {
    '=', ',', '.', '!', '(', ')', '[', ']', '{', '}', ':', ';', '+', '-', '<', '>', '*', '@', '"', '\''
};

fn is_punctuation(c: char) -> bool {
    PUNCTUATIONS.contains(&c)
}

fn is_letter(c: char) -> bool {
    !c.is_whitespace() && !is_punctuation(c)
}

pub struct Lexer<'a> {
    reader: CharReader<&'a File>,

    last_token_kind: Option<TokenKind>,

    curr_char: Option<char>,
    spelling: String,

    begin_pos: CursorPos,
    last_pos: CursorPos,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a File) -> Self {
        let mut reader = CharReader::new(input);
        let curr_char = reader.read_char();
        return Self {
            reader,
            last_token_kind: None,
            curr_char,
            spelling: String::new(),
            begin_pos: (1, 1),
            last_pos: (1, 0),
        };
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_nontokens();
        self.reset_begin();
        self.spelling.clear();

        let kind = self.token_kind();
        let token = Token::new(
            kind,
            mem::take(&mut self.spelling),
            self.begin_pos,
            self.last_pos,
        );

        self.last_token_kind = Some(kind);
        return token;
    }

    fn accept_char(&mut self) {
        if let Some(c) = self.curr_char {
            self.spelling.push(c);
        }

        self.skip_char();
    }

    fn skip_char(&mut self) {
        if self.curr_char == Some(EOL) {
            self.last_pos.0 += 1;
            self.last_pos.1 = 0;
        } else {
            self.last_pos.1 += 1;
        }

        self.curr_char = self.reader.read_char();
    }

    fn inspect_char(&self, n: usize) -> Option<char> {
        self.reader.inspect(n)
    }

    fn reset_begin(&mut self) {
        self.begin_pos = (self.last_pos.0, self.last_pos.1 + 1);
    }

    fn skip_nontokens(&mut self) {
        loop {
            match self.curr_char {
                Some(c) if c.is_whitespace() => self.skip_char(),
                Some('/') => match self.inspect_char(0) {
                    Some('/') => loop {
                        self.skip_char();
                        match self.curr_char {
                            None | Some(EOL) => break,
                            _ => (),
                        };
                    },
                    Some('*') => {
                        self.skip_char();
                        loop {
                            if self.curr_char.is_some() {
                                self.skip_char();
                                if self.curr_char == Some('*') && self.inspect_char(0) == Some('/')
                                {
                                    break;
                                }
                            } else {
                                panic!("unterminated comment");
                            }
                        }
                        self.skip_char();
                        self.skip_char();
                    }
                    _ => (),
                },
                _ => break,
            }
        }
    }

    fn token_kind(&mut self) -> TokenKind {
        match self.curr_char {
            None => TokenKind::EOF,
            Some(':') => {
                self.accept_char();
                TokenKind::Colon
            }
            Some(';') => {
                self.accept_char();
                TokenKind::Semicolon
            }
            Some(',') => {
                self.accept_char();
                TokenKind::Comma
            }
            Some('.') => {
                self.accept_char();

                if self.curr_char == Some('.') {
                    self.accept_char();

                    if self.curr_char == Some('.') {
                        self.accept_char();
                        return TokenKind::Ellipsis;
                    }

                    return TokenKind::To;
                }

                if matches!(
                    self.last_token_kind,
                    Some(TokenKind::Identifier | TokenKind::RightParen | TokenKind::StrLiteral)
                ) {
                    TokenKind::Dot
                } else {
                    self.extract_fraction()
                }
            }
            Some('=') => {
                self.accept_char();
                if self.curr_char == Some('=') {
                    self.accept_char();
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            Some('{') => {
                self.accept_char();
                TokenKind::LeftBrace
            }
            Some('}') => {
                self.accept_char();
                TokenKind::RightBrace
            }
            Some('(') => {
                self.accept_char();
                TokenKind::LeftParen
            }
            Some(')') => {
                self.accept_char();
                TokenKind::RightParen
            }
            Some('[') => {
                self.accept_char();
                TokenKind::LeftBracket
            }
            Some(']') => {
                self.accept_char();
                TokenKind::RightBracket
            }
            Some('<') => {
                self.accept_char();
                TokenKind::LeftChevron
            }
            Some('>') => {
                self.accept_char();
                TokenKind::RightChevron
            }
            Some('+') => {
                self.accept_char();
                if self.curr_char == Some('+') {
                    self.accept_char();
                    TokenKind::Increment
                } else {
                    TokenKind::Plus
                }
            }
            Some('-') => {
                self.accept_char();
                if self.curr_char == Some('>') {
                    self.accept_char();
                    TokenKind::Arrow
                } else if self.curr_char == Some('-') {
                    TokenKind::Decrement
                } else {
                    TokenKind::Minus
                }
            }
            Some('*') => {
                self.accept_char();
                TokenKind::Multiply
            }
            Some('/') => {
                self.accept_char();
                TokenKind::Divide
            }
            Some('!') => {
                self.accept_char();
                if self.curr_char == Some('=') {
                    self.accept_char();
                    TokenKind::NotEqual
                } else {
                    TokenKind::Not
                }
            }
            Some('"') => {
                self.accept_char();
                loop {
                    if self.curr_char == Some('\\') {
                        self.skip_char();
                        if self.extract_escape() {
                            continue;
                        }
                    }

                    if self.curr_char.is_none() || self.curr_char == Some('\n') {
                        panic!("unterminated string");
                    }

                    if self.curr_char == Some('"') {
                        break;
                    }

                    self.accept_char();
                }
                self.accept_char();
                TokenKind::StrLiteral
            }
            Some('\'') => {
                self.accept_char();
                loop {
                    if self.curr_char == Some('\\') {
                        self.skip_char();
                        if self.extract_escape() {
                            continue;
                        }
                    }

                    if self.curr_char.is_none() || self.curr_char == Some('\n') {
                        panic!("unterminated string");
                    }

                    if self.curr_char == Some('\'') {
                        break;
                    }

                    self.accept_char();
                }
                self.accept_char();
                TokenKind::CharLiteral
            }
            Some(c) if c.is_numeric() => {
                loop {
                    self.accept_char();
                    match self.curr_char {
                        Some(c) if c.is_numeric() => (),
                        _ => break,
                    }
                }

                match self.curr_char {
                    Some('.') => {
                        if self.inspect_char(0) != Some('.') {
                            self.extract_fraction()
                        } else {
                            TokenKind::IntLiteral
                        }
                    }
                    Some('e' | 'E') => self.extract_fraction(),
                    _ => TokenKind::IntLiteral,
                }
            }
            Some(c) if is_letter(c) => {
                loop {
                    match self.curr_char {
                        Some(c) if is_letter(c) => self.accept_char(),
                        _ => break,
                    }
                }

                if self.spelling == "true" || self.spelling == "false" {
                    TokenKind::BoolLiteral
                } else if let Some(keyword) = self.extract_keyword() {
                    keyword
                } else if self.spelling == "true" || self.spelling == "false" {
                    TokenKind::BoolLiteral
                } else {
                    TokenKind::Identifier
                }
            }
            _ => {
                self.accept_char();
                TokenKind::Unknown
            }
        }
    }

    fn extract_keyword(&self) -> Option<TokenKind> {
        match self.spelling.as_str() {
            "extern" => Some(TokenKind::Extern),
            "export" => Some(TokenKind::Export),
            "import" => Some(TokenKind::Import),
            "let" => Some(TokenKind::Let),
            "mut" => Some(TokenKind::Mut),
            "restrict" => Some(TokenKind::Restrict),
            "return" => Some(TokenKind::Return),
            "type" => Some(TokenKind::Type),

            "i8" => Some(TokenKind::I8),
            "i32" => Some(TokenKind::I32),
            "i64" => Some(TokenKind::I64),
            _ => None,
        }
    }

    fn extract_fraction(&mut self) -> TokenKind {
        if self.curr_char == Some('.') {
            loop {
                self.accept_char();
                match self.curr_char {
                    Some(c) if c.is_numeric() => (),
                    _ => break,
                }
            }
        }

        if matches!(self.curr_char, Some('e' | 'E')) {
            let c = self.inspect_char(0);
            let nc = self.inspect_char(1);

            if c.is_some() && c.unwrap().is_numeric()
                || (matches!(c, Some('+' | '-')) && nc.is_some() && nc.unwrap().is_numeric())
            {
                self.accept_char();
                loop {
                    self.accept_char();
                    if self.curr_char.is_none() || !self.curr_char.unwrap().is_numeric() {
                        break;
                    }
                }
            }
        }

        if self.spelling.len() <= 1 {
            TokenKind::Unknown
        } else {
            TokenKind::FloatLiteral
        }
    }

    fn extract_escape(&mut self) -> bool {
        match self.curr_char {
            Some('n') => {
                self.skip_char();
                self.spelling.push('\n');
                true
            }
            Some('r') => {
                self.skip_char();
                self.spelling.push('\r');
                true
            }
            Some('t') => {
                self.skip_char();
                self.spelling.push('\t');
                true
            }
            Some('\\') => {
                self.skip_char();
                self.spelling.push('\\');
                true
            }
            Some('\'') => {
                self.skip_char();
                self.spelling.push('\'');
                true
            }
            Some('"') => {
                self.skip_char();
                self.spelling.push('\"');
                true
            }
            _ => false,
        }
    }
}
