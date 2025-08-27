use crate::compiler::tokens::{IntegerFormat, Token};

pub struct Parser {
    state: State,
    next_id: NodeId,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
            next_id: NodeId { inner: 0 },
        }
    }

    pub fn process_token(&mut self, token: Token) -> Option<SyntaxNode> {
        let id = self.next_id;
        self.next_id.inner += 1;

        let kind = match (&self.state, token) {
            (State::Initial, Token::Binding) => {
                // Not supported yet; ignore for now.
                return None;
            }
            (State::Initial, Token::Comment { text }) => {
                NodeKind::Comment { text }
            }
            (State::Initial, Token::Identifier { name }) => {
                NodeKind::Identifier { name }
            }
            (State::Initial, Token::Integer { value, format }) => {
                NodeKind::Integer { value, format }
            }
            (_, token) => {
                panic!("Unexpected token `{token:?}`");
            }
        };

        Some(SyntaxNode { id, kind })
    }
}

enum State {
    Initial,
}

pub struct SyntaxNode {
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    pub inner: u64,
}

pub enum NodeKind {
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}
