use crate::{
    node::{Node, Nodes},
    stack::{Stack, StackIsEmpty},
};

pub fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut nodes = Nodes::new();

    for ch in code.chars() {
        if !ch.is_whitespace() {
            nodes.last.push(ch);
        }

        let node = match nodes.last.as_str() {
            "=" => Node::Equals,
            "assert" => Node::Assert,
            "1" => Node::Integer { value: 1 },
            "2" => Node::Integer { value: 2 },
            _ => {
                // The last node is already treated as an unknown identifier.
                // Since we haven't figured out yet what it's supposed to be, we
                // can keep it that way.
                continue;
            }
        };

        nodes.inner.push(node);
        nodes.last.clear();
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
            Node::Integer { value } => {
                stack.push(value);
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
