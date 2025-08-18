use std::mem;

use crate::compiler::input_code::InputCode;

pub struct Tokenizer {}

impl Tokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_token(
        &mut self,
        input_code: &mut InputCode,
    ) -> Option<Token> {
        let mut token = String::new();

        while let Some(ch) = input_code.next() {
            match ch {
                '#' => {
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
                ch if ch.is_whitespace() => {
                    let token = if let Ok(value) = token.parse() {
                        Token::Number { value }
                    } else {
                        Token::Identifier {
                            name: mem::take(&mut token),
                        }
                    };

                    return Some(token);
                }
                ch => {
                    token.push(ch);
                }
            }
        }

        None
    }

    pub fn process_all_tokens(mut self, input_code: InputCode) -> Vec<Token> {
        let mut input_code = input_code;
        let mut tokens = Vec::new();

        while let Some(token) = self.process_token(&mut input_code) {
            tokens.push(token);
        }

        tokens
    }
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
