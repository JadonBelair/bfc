use std::{iter::Enumerate, str::Chars};

pub mod token;

pub struct Lexer<'a> {
    source: Enumerate<Chars<'a>>,
    current_char: char,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut l = Self {
            source: source.chars().enumerate(),
            current_char: '\0',
            current_pos: 0,
        };

        l.next();

        return l;
    }

    fn is_valid_char(&self) -> bool {
        self.current_char == '+'
            || self.current_char == '-'
            || self.current_char == '<'
            || self.current_char == '>'
            || self.current_char == '.'
            || self.current_char == ','
            || self.current_char == '['
            || self.current_char == ']'
    }

    fn next(&mut self) {
        if let Some((i, c)) = self.source.next() {
            self.current_char = c;
            self.current_pos = i;

            if !self.is_valid_char() {
                self.next();
            }
        } else {
            self.current_char = '\0';
        }
    }
}
