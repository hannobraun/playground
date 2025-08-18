pub fn tokenize(input_code: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for token in input_code.split_whitespace() {
        let token = token.to_string();
        tokens.push(token);
    }

    tokens
}
