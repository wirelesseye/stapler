use std::{fs::File, mem};

use crate::{char_reader::{CharReader, EOL}, token::{Token, TokenKind, CursorPos}};

pub struct Lexer<'a> {
    reader: CharReader<&'a File>,
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
                Some('/') => {
                    match self.inspect_char(0) {
                        Some('/') => {
                            loop {
                                self.skip_char();
                                match self.curr_char {
                                    None | Some(EOL) => break,
                                    _ => (),
                                };
                            }
                        },
                        Some('*') => {
                            self.skip_char();
                            loop {
                                if self.curr_char.is_some() {
                                    self.skip_char();
                                    if self.curr_char == Some('*') && self.inspect_char(0) == Some('/') {
                                        break;
                                    }
                                } else {
                                    panic!("unterminated comment");
                                }
                            };
                            self.skip_char();
                            self.skip_char();
                        },
                        _ => (),
                    }
                }
                _ => break,
            }
        }
    }

    fn token_kind(&mut self) -> TokenKind {
        match self.curr_char {
            Some(_) => {
                self.accept_char();
                TokenKind::Unknown
            },
            None => TokenKind::EOF,
        }
    }

    fn long_token_kind(&mut self) -> TokenKind {
        return TokenKind::Unknown;
    }
}