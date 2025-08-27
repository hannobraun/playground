use crate::compiler::tokens::Token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_token(&mut self, token: Token) -> SyntaxElement {
        SyntaxElement {
            kind: SyntaxElementKind::UnprocessedToken { token },
        }
    }
}

pub struct SyntaxElement {
    pub kind: SyntaxElementKind,
}

pub enum SyntaxElementKind {
    UnprocessedToken { token: Token },
}
