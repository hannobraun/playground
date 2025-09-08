use std::mem;

use crate::compiler::code::tokens::{IntegerFormat, Token};

pub struct Parser {
    state: Vec<State>,
    next_id: NodeId,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            next_id: NodeId { inner: 1 }, // ID `0` is reserved for root block
        }
    }

    pub fn process_token(&mut self, token: Token) -> Option<Node> {
        let id = self.next_id;
        self.next_id.inner += 1;

        let kind = match (self.state.last_mut(), token) {
            (Some(State::Block { nodes }), Token::BlockClose) => {
                let nodes = mem::take(nodes);

                self.state.pop();
                NodeKind::Block { nodes }
            }
            (None, token) => {
                let (kind, state) = process_token_in_block(token);

                if let Some(state) = state {
                    self.state.push(state);
                }

                kind?
            }
            (Some(State::Block { nodes }), token) => {
                let (kind, state) = process_token_in_block(token);

                if let Some(kind) = kind {
                    nodes.push(Node { id, kind });
                }
                if let Some(state) = state {
                    self.state.push(state);
                }

                return None;
            }
            (Some(State::Binding { names }), Token::Identifier { name }) => {
                names.push(name);
                return None;
            }
            (Some(State::Binding { names }), Token::Terminator) => {
                let names = mem::take(names);

                self.state.pop();
                NodeKind::Binding { names }
            }
            (_, token) => {
                panic!("Unexpected token `{token:?}`");
            }
        };

        Some(Node { id, kind })
    }
}

enum State {
    Binding { names: Vec<String> },
    Block { nodes: Vec<Node> },
}

fn process_token_in_block(token: Token) -> (Option<NodeKind>, Option<State>) {
    let node = match token {
        Token::Binding => {
            return (None, Some(State::Binding { names: Vec::new() }));
        }
        Token::BlockOpen => {
            return (None, Some(State::Block { nodes: Vec::new() }));
        }
        Token::Comment { text } => NodeKind::Comment { text },
        Token::Identifier { name } => NodeKind::Identifier { name },
        Token::Integer { value, format } => NodeKind::Integer { value, format },

        token => {
            panic!("Unexpected token `{token:?}`");
        }
    };

    (Some(node), None)
}

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    pub inner: u64,
}

#[derive(Debug)]
pub enum NodeKind {
    Binding { names: Vec<String> },
    Block { nodes: Vec<Node> },
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}
