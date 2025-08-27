use crate::compiler::tokens::Token;

pub struct Parser {
    pub next_id: SyntaxElementId,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            next_id: SyntaxElementId { inner: 0 },
        }
    }

    pub fn process_token(&mut self, token: Token) -> SyntaxElement {
        let id = self.next_id;
        self.next_id.inner += 1;

        SyntaxElement {
            id,
            kind: SyntaxElementKind::UnprocessedToken { token },
        }
    }
}

pub struct SyntaxElement {
    pub id: SyntaxElementId,
    pub kind: SyntaxElementKind,
}

#[derive(Clone, Copy)]
pub struct SyntaxElementId {
    pub inner: u64,
}

pub enum SyntaxElementKind {
    UnprocessedToken { token: Token },
}
