use std::{iter::Peekable, mem, str::Chars};

pub struct Tokenizer<'a> {
    pub chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input_code: &'a str) -> Self {
        Self {
            chars: input_code.chars().peekable(),
        }
    }

    pub fn process_token(&mut self) -> Option<Token> {
        let mut token = String::new();

        while let Some(ch) = self.chars.next() {
            match ch {
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
                '#' => {
                    while let Some(&ch) = self.chars.peek() {
                        // This would be redundant, if we handled multiple
                        // subsequent whitespace characters correctly.
                        self.chars.next();

                        if ch == '\n' {
                            break;
                        } else {
                            self.chars.next();
                            continue;
                        }
                    }
                }
                ch => {
                    token.push(ch);
                }
            }
        }

        None
    }

    pub fn process_all_tokens(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.process_token() {
            tokens.push(token);
        }

        tokens
    }
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
