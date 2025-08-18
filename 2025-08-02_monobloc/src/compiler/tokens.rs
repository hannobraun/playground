pub fn tokenize(input_code: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for token in input_code.split_whitespace() {
        tokens.push(token.to_string());
    }

    tokens
}
