use std::{iter::Peekable, mem, str::Chars};

pub fn tokenize(input_code: &str) -> Vec<Token> {
    let tokenizer = Tokenizer {
        chars: input_code.chars().peekable(),
        tokens: Vec::new(),
    };

    tokenizer.process_all_tokens()
}

pub struct Tokenizer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
}

impl Tokenizer<'_> {
    pub fn process_token(&mut self) -> Option<Token> {
        let mut token = String::default();

        while let Some(ch) = self.chars.next() {
            if ch.is_whitespace() {
                let token = if let Ok(value) = token.parse() {
                    Token::Number { value }
                } else {
                    Token::Identifier {
                        name: mem::take(&mut token),
                    }
                };

                return Some(token);
            } else if ch == '#' {
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
            } else {
                token.push(ch);
            }
        }

        None
    }

    pub fn process_all_tokens(mut self) -> Vec<Token> {
        while let Some(token) = self.process_token() {
            self.tokens.push(token);
        }

        self.tokens
    }
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
