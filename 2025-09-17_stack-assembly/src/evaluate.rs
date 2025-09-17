use crate::{
    nodes::{Node, Nodes},
    stack::{Stack, StackIsEmpty},
};

pub fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut nodes = Nodes::new();

    for ch in code.chars() {
        let mut token = nodes.last.to_string();

        if !ch.is_whitespace() {
            token.push(ch);
        }

        let node = match token.as_str() {
            "=" => Node::Equals,
            "assert" => Node::Assert,
            "nop" => Node::Nop,
            "1" => Node::Integer { value: 1 },
            "2" => Node::Integer { value: 2 },
            _ => {
                // The last node is already treated as an unknown identifier.
                // Since we haven't figured out yet what it's supposed to be, we
                // can keep it that way.
                nodes.last = Node::UnknownIdentifier { name: token };
                continue;
            }
        };

        if ch.is_whitespace() {
            nodes.inner.push(node);
            nodes.last = Node::Empty;
        } else {
            nodes.last = node;
        }
    }

    let mut stack = Stack::new();

    for node in nodes {
        match node {
            Node::Assert => {
                let a = stack.pop()?;

                if a == 0 {
                    return Err(EvaluateError::Other);
                }
            }
            Node::Equals => {
                let b = stack.pop()?;
                let a = stack.pop()?;

                match a == b {
                    false => {
                        stack.push(0);
                    }
                    true => {
                        stack.push(1);
                    }
                }
            }
            Node::Nop => {
                // does nothing
            }

            Node::Integer { value } => {
                stack.push(value);
            }

            Node::Empty => {
                // no effect at runtime
            }
            Node::UnknownIdentifier { name } => {
                let _ = name;
                return Err(EvaluateError::Other);
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluateError {
    #[error(transparent)]
    StackIsEmpty(#[from] StackIsEmpty),

    #[error("Other error while evaluating")]
    Other,
}
