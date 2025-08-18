use std::mem;

pub fn tokenize(input_code: &str) -> Vec<Token> {
    let mut chars = input_code.chars().peekable();
    let mut current_token = String::default();
    let mut tokens = Vec::new();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            if let Ok(value) = current_token.parse() {
                tokens.push(Token::Number { value });
            } else {
                tokens.push(Token::Identifier {
                    name: mem::take(&mut current_token),
                });
            }
        } else if ch == '#' {
            while let Some(&ch) = chars.peek() {
                // This would be redundant, if we handled multiple subsequent
                // whitespace characters correctly.
                chars.next();

                if ch == '\n' {
                    break;
                } else {
                    chars.next();
                    continue;
                }
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
