pub fn tokenize() -> Vec<Token> {
    vec![
        Token::Fun,
        Token::Identifier {
            value: "start".to_string(),
        },
        Token::Literal { value: 42 },
    ]
}

pub enum Token {
    Fun,
    Identifier { value: String },
    Literal { value: i32 },
}
