use std::mem;

use crate::compiler::code::{
    nodes::{Node, NodeKind, Nodes},
    tokens::Token,
};

pub struct Parser {
    state: Vec<State>,
}

impl Parser {
    pub fn new() -> Self {
        Self { state: Vec::new() }
    }

    pub fn process_token(
        &mut self,
        token: Token,
        nodes: &mut Nodes,
    ) -> Option<Node> {
        let id = nodes.next_id();

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
