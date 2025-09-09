use std::mem;

use crate::compiler::code::{
    nodes::{Block, Node, NodeKind, Nodes},
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
            (Some(State::Block { block }), Token::BlockClose) => {
                let block = mem::take(block);

                self.state.pop();
                NodeKind::Block { block }
            }
            (None, token) => {
                let (kind, state) = process_token_in_block(token);

                if let Some(state) = state {
                    self.state.push(state);
                }

                kind?
            }
            (Some(State::Block { block }), token) => {
                let (kind, state) = process_token_in_block(token);

                if let Some(kind) = kind {
                    block.nodes.push(Node { id, kind });
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
    Block { block: Block },
}

fn process_token_in_block(token: Token) -> (Option<NodeKind>, Option<State>) {
    let node = match token {
        Token::Binding => {
            return (None, Some(State::Binding { names: Vec::new() }));
        }
        Token::BlockOpen => {
            return (
                None,
                Some(State::Block {
                    block: Block { nodes: Vec::new() },
                }),
            );
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
