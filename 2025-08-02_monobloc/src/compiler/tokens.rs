use std::mem;

pub fn tokenize(input_code: &str) -> Vec<Token> {
    let chars = input_code.chars().peekable();
    let mut current_token = String::default();
    let mut tokens = Vec::new();

    for ch in chars {
        if ch.is_whitespace() {
            if let Ok(value) = current_token.parse() {
                tokens.push(Token::Number { value });
            } else {
                tokens.push(Token::Identifier {
                    name: mem::take(&mut current_token),
                });
            }
        } else {
            current_token.push(ch);
        }
    }

    tokens
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
