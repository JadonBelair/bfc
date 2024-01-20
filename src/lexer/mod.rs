use std::{iter::Enumerate, str::Chars};

use crate::lexer::token::{Token, TokenType};

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

    pub fn lex(&mut self) -> Vec<Token> {
        let mut jump_stack = Vec::new();
        let mut tokens = Vec::new();

        while self.current_char != '\0' {
            match self.current_char {
                '+' | '-' | '<' | '>' | '.' | ',' => {
                    let matching_char = self.current_char;
                    let token_type = TokenType::from(matching_char);
                    let mut amount = 1;

                    self.next();

                    while self.current_char == matching_char {
                        self.next();
                        amount += 1;
                    }

                    tokens.push(Token { token_type, amount });
                },
                '[' => {
                    jump_stack.push(tokens.len());
                    tokens.push(Token {
                        token_type: TokenType::JumpIfZero,
                        amount: 0,
                    });
                    self.next();
                },
                ']' => {
                    if let Some(pos) = jump_stack.pop() {
                        let diff = tokens.len() - pos;

                        tokens.push(Token {
                            token_type: TokenType::JumpIfNotZero,
                            amount: diff - 1,
                        });

                        tokens[pos].amount = diff + 1;
                    }
                    self.next();
                },
                _ => ()
            }
        }

        return tokens;
    }
}
