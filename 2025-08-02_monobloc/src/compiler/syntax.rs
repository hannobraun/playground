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

        let kind = match token {
            Token::Identifier { name } => {
                SyntaxElementKind::Identifier { name }
            }
            token => SyntaxElementKind::UnprocessedToken { token },
        };

        SyntaxElement { id, kind }
    }
}

pub struct SyntaxElement {
    pub id: SyntaxElementId,
    pub kind: SyntaxElementKind,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct SyntaxElementId {
    pub inner: u64,
}

pub enum SyntaxElementKind {
    Identifier { name: String },
    UnprocessedToken { token: Token },
}
