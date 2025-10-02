pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();

    for token in input.split_whitespace() {
        let token = Token::is_reference(token)
            .or_else(|| Token::is_label(token))
            .unwrap_or_else(|| Token::operator(token));

        tokens.push(token);
    }

    tokens
}

pub enum Token<'r> {
    Label { name: &'r str },
    Operator { name: &'r str },
    Reference { name: &'r str },
}

impl<'r> Token<'r> {
    fn is_label(token: &'r str) -> Option<Self> {
        if let Some((name, "")) = token.rsplit_once(":") {
            Some(Self::Label { name })
        } else {
            None
        }
    }

    fn is_reference(token: &'r str) -> Option<Self> {
        if let Some(("", name)) = token.split_once("@") {
            Some(Self::Reference { name })
        } else {
            None
        }
    }

    fn operator(name: &'r str) -> Self {
        Self::Operator { name }
    }
}
