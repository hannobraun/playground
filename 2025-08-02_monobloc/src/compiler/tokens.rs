pub fn tokenize(input_code: &str) -> Vec<String> {
    input_code
        .split_whitespace()
        .map(|token| token.to_string())
        .collect()
}
