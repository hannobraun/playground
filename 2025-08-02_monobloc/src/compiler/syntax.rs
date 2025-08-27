use crate::compiler::tokens::Token;

pub struct Parser {
    pub next_id: u64,
}

impl Parser {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn process_token(&mut self, token: Token) -> SyntaxElement {
        let id = self.next_id;
        self.next_id += 1;

        SyntaxElement {
            id,
            kind: SyntaxElementKind::UnprocessedToken { token },
        }
    }
}

pub struct SyntaxElement {
    pub id: u64,
    pub kind: SyntaxElementKind,
}

pub enum SyntaxElementKind {
    UnprocessedToken { token: Token },
}
