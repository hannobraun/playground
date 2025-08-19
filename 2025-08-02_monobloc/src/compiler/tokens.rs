use std::mem;

use crate::compiler::input_code::InputCode;

pub struct Tokenizer {
    state: State,
    buf: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
            buf: String::new(),
        }
    }

    #[cfg(test)]
    pub fn process_all_tokens(mut self, input_code: InputCode) -> Vec<Token> {
        let mut input_code = input_code;
        let mut tokens = Vec::new();

        while let Some(token) = self.process_token(&mut input_code) {
            tokens.push(token);
        }

        tokens
    }

    #[cfg(test)]
    pub fn process_token(
        &mut self,
        input_code: &mut InputCode,
    ) -> Option<Token> {
        loop {
            match self.process_char(input_code) {
                ProcessCharOutcome::NoMoreChars => {
                    return None;
                }
                ProcessCharOutcome::TokenIsReady { token } => {
                    return Some(token);
                }
                ProcessCharOutcome::TokenNotReady { ch: _ } => {
                    continue;
                }
            }
        }
    }

    pub fn process_char(
        &mut self,
        input_code: &mut InputCode,
    ) -> ProcessCharOutcome {
        let Some(ch) = input_code.next() else {
            return ProcessCharOutcome::NoMoreChars;
        };

        match (&self.state, ch) {
            (State::Initial, '#') => {
                self.state = State::Comment;
            }
            (State::Initial, ch) if ch.is_whitespace() => {
                let token = if let Ok(value) = self.buf.parse() {
                    Token::Number { value }
                } else {
                    Token::Identifier {
                        name: mem::take(&mut self.buf),
                    }
                };

                return ProcessCharOutcome::TokenIsReady { token };
            }
            (State::Initial, ch) => {
                self.buf.push(ch);
            }
            (State::Comment, '\n') => {
                self.state = State::Initial;

                return ProcessCharOutcome::TokenIsReady {
                    token: Token::Comment {
                        text: mem::take(&mut self.buf),
                    },
                };
            }
            (State::Comment, ch) => {
                self.buf.push(ch);
            }
        }

        ProcessCharOutcome::TokenNotReady { ch }
    }
}

pub enum State {
    Initial,
    Comment,
}

#[derive(Debug)]
pub enum Token {
    Comment { text: String },
    Identifier { name: String },
    Number { value: i32 },
}

#[derive(Debug)]
pub enum ProcessCharOutcome {
    NoMoreChars,
    TokenIsReady { token: Token },
    TokenNotReady { ch: char },
}
