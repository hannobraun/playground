pub fn tokenize(input_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in input_code.split_whitespace() {
        if let Ok(value) = token.parse() {
            tokens.push(Token::Number { value });
        } else {
            tokens.push(Token::Identifier {
                name: token.to_string(),
            });
        }
    }

    tokens
}

pub enum Token {
    Identifier { name: String },
    Number { value: i32 },
}
