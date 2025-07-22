pub fn tokenize() -> Vec<Token> {
    let code = "fun start 42";

    let mut tokens = Vec::new();

    for token in code.split_whitespace() {
        let token = match token {
            "fun" => Token::Fun,
            word => {
                if let Ok(value) = word.parse::<i32>() {
                    Token::Literal { value }
                } else {
                    Token::Identifier {
                        value: word.to_string(),
                    }
                }
            }
        };

        tokens.push(token);
    }

    tokens
}

pub enum Token {
    Fun,
    Identifier { value: String },
    Literal { value: i32 },
}
