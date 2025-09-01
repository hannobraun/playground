use std::mem;

use crate::compiler::tokens::{IntegerFormat, Token};

pub struct Parser {
    state: Vec<State>,
    next_id: NodeId,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: vec![State::Initial],
            next_id: NodeId { inner: 0 },
        }
    }

    pub fn process_token(&mut self, token: Token) -> Option<SyntaxNode> {
        let id = self.next_id;
        self.next_id.inner += 1;

        let kind = match (self.state.last_mut(), token) {
            (Some(State::Initial), Token::Binding) => {
                self.state.pop();
                self.state.push(State::Binding { names: Vec::new() });
                return None;
            }
            (Some(State::Initial), Token::Comment { text }) => {
                NodeKind::Comment { text }
            }
            (Some(State::Initial), Token::Identifier { name }) => {
                NodeKind::Identifier { name }
            }
            (Some(State::Initial), Token::Integer { value, format }) => {
                NodeKind::Integer { value, format }
            }
            (Some(State::Binding { names }), Token::Identifier { name }) => {
                names.push(name);
                return None;
            }
            (Some(State::Binding { names }), Token::Terminator) => {
                let names = mem::take(names);

                self.state.pop();
                self.state.push(State::Initial);
                NodeKind::Binding { names }
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
    Binding { names: Vec<String> },
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
    Binding { names: Vec<String> },
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}
