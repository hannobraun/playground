use crate::compiler::tokens::Token;

pub struct Parser {
    pub next_id: NodeId,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            next_id: NodeId { inner: 0 },
        }
    }

    pub fn process_token(&mut self, token: Token) -> SyntaxNode {
        let id = self.next_id;
        self.next_id.inner += 1;

        let kind = match token {
            Token::Identifier { name } => {
                SyntaxElementKind::Identifier { name }
            }
            token => SyntaxElementKind::UnprocessedToken { token },
        };

        SyntaxNode { id, kind }
    }
}

pub struct SyntaxNode {
    pub id: NodeId,
    pub kind: SyntaxElementKind,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    pub inner: u64,
}

pub enum SyntaxElementKind {
    Identifier { name: String },
    UnprocessedToken { token: Token },
}
