use std::io::{Read, BufReader, BufRead};

pub const EOL: char = '\n';

pub struct CharReader<R: Read> {
    inner_reader: BufReader<R>,
    buff: String,
    pos: usize,
}

impl<R: Read> CharReader<R> {
    pub fn new(inner: R) -> Self {
        let mut inner_reader = BufReader::new(inner);
        let mut buff = String::new();
        inner_reader.read_line(&mut buff).unwrap();

        Self {
            inner_reader,
            buff,
            pos: 0,
        }
    }

    pub fn read_char(&mut self) -> Option<char> {
        match self.buff.chars().nth(self.pos) {
            Some('\n' | '\r') | None => {
                if self.next_line() {
                    Some(EOL)
                } else {
                    None
                }
            },
            Some(c) => {
                self.pos += 1;
                Some(c)
            }
        }
    }

    pub fn inspect(&self, n: usize) -> Option<char> {
        self.buff.chars().nth(self.pos + n)
    }

    pub fn next_line(&mut self) -> bool {
        self.buff.clear();
        self.pos = 0;
        return self.inner_reader.read_line(&mut self.buff).unwrap() != 0;
    }
}
