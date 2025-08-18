use std::{iter::Peekable, mem, str::Chars};

pub fn tokenize(input_code: &str) -> Vec<Token> {
    let mut tokenizer = Tokenizer {
        chars: input_code.chars().peekable(),
        current_token: String::default(),
        tokens: Vec::new(),
    };

    tokenizer.process_all_tokens();

    tokenizer.tokens
}

pub struct Tokenizer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub current_token: String,
    pub tokens: Vec<Token>,
}

impl Tokenizer<'_> {
    pub fn process_all_tokens(&mut self) {
        while let Some(ch) = self.chars.next() {
            if ch.is_whitespace() {
                if let Ok(value) = self.current_token.parse() {
                    self.tokens.push(Token::Number { value });
                } else {
                    self.tokens.push(Token::Identifier {
                        name: mem::take(&mut self.current_token),
                    });
                }
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
                self.current_token.push(ch);
            }
        }
    }
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
