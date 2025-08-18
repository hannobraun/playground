pub fn tokenize(input_code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in input_code.split_whitespace() {
        let token = Token::Identifier {
            name: token.to_string(),
        };
        tokens.push(token);
    }

    tokens
}

pub enum Token {
    Identifier { name: String },
}
