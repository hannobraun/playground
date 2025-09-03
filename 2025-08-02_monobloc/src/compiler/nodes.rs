use std::mem;

use crate::compiler::tokens::{IntegerFormat, Token};

pub struct Parser {
    state: Vec<State>,
    next_id: NodeId,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            next_id: NodeId { inner: 0 },
        }
    }

    pub fn process_token(&mut self, token: Token) -> Option<Node> {
        let id = self.next_id;
        self.next_id.inner += 1;

        let kind = match (self.state.last_mut(), token) {
            (Some(State::Block), Token::BlockClose) => {
                self.state.pop();
                NodeKind::Block
            }
            (None | Some(State::Block), token) => {
                process_token_in_block(token, &mut self.state)?
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
    Block,
}

fn process_token_in_block(
    token: Token,
    state: &mut Vec<State>,
) -> Option<NodeKind> {
    let node = match token {
        Token::Binding => {
            state.push(State::Binding { names: Vec::new() });
            return None;
        }
        Token::BlockOpen => {
            state.push(State::Block);
            return None;
        }
        Token::Comment { text } => NodeKind::Comment { text },
        Token::Identifier { name } => NodeKind::Identifier { name },
        Token::Integer { value, format } => NodeKind::Integer { value, format },

        token => {
            panic!("Unexpected token `{token:?}`");
        }
    };

    Some(node)
}

pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct NodeId {
    pub inner: u64,
}

pub enum NodeKind {
    Binding { names: Vec<String> },
    Block,
    Comment { text: String },
    Identifier { name: String },
    Integer { value: u32, format: IntegerFormat },
}
