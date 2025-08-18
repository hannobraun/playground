use std::mem;

use crate::compiler::input_code::InputCode;

pub struct Tokenizer {
    state: State,
    token: String,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
            token: String::new(),
        }
    }

    pub fn process_all_tokens(mut self, input_code: InputCode) -> Vec<Token> {
        let mut input_code = input_code;
        let mut tokens = Vec::new();

        while let Some(token) = self.process_token(&mut input_code) {
            tokens.push(token);
        }

        tokens
    }

    pub fn process_token(
        &mut self,
        input_code: &mut InputCode,
    ) -> Option<Token> {
        while let Some(ch) = input_code.next() {
            match (&self.state, ch) {
                (State::Initial, '#') => {
                    while let Some(&ch) = input_code.peek() {
                        // This would be redundant, if we handled multiple
                        // subsequent whitespace characters correctly.
                        input_code.next();

                        if ch == '\n' {
                            break;
                        } else {
                            input_code.next();
                            continue;
                        }
                    }
                }
                (State::Initial, ch) if ch.is_whitespace() => {
                    let token = if let Ok(value) = self.token.parse() {
                        Token::Number { value }
                    } else {
                        Token::Identifier {
                            name: mem::take(&mut self.token),
                        }
                    };

                    return Some(token);
                }
                (State::Initial, ch) => {
                    self.token.push(ch);
                }
            }
        }

        None
    }
}

enum State {
    Initial,
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
