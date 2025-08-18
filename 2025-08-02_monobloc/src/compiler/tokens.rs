use std::{iter::Peekable, mem, str::Chars};

pub fn tokenize(input_code: &str) -> Vec<Token> {
    let chars = input_code.chars().peekable();
    let current_token = String::default();
    let tokens = Vec::new();

    let mut tokenizer = Tokenizer {
        chars,
        current_token,
        tokens,
    };

    while let Some(ch) = tokenizer.chars.next() {
        if ch.is_whitespace() {
            if let Ok(value) = tokenizer.current_token.parse() {
                tokenizer.tokens.push(Token::Number { value });
            } else {
                tokenizer.tokens.push(Token::Identifier {
                    name: mem::take(&mut tokenizer.current_token),
                });
            }
        } else if ch == '#' {
            while let Some(&ch) = tokenizer.chars.peek() {
                // This would be redundant, if we handled multiple subsequent
                // whitespace characters correctly.
                tokenizer.chars.next();

                if ch == '\n' {
                    break;
                } else {
                    tokenizer.chars.next();
                    continue;
                }
            }
        } else {
            tokenizer.current_token.push(ch);
        }
    }

    tokenizer.tokens
}

pub struct Tokenizer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub current_token: String,
    pub tokens: Vec<Token>,
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
