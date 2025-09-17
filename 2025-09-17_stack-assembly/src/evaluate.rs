use crate::{
    nodes::{Node, Nodes},
    stack::{Stack, StackIsEmpty},
};

pub fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut nodes = Nodes::new();

    for ch in code.chars() {
        let mut token = nodes.last.to_string();

        let finalize = if !ch.is_whitespace() {
            token.push(ch);
            false
        } else {
            true
        };

        let node = match token.as_str() {
            "=" => Node::Equals,
            "assert" => Node::Assert,
            "nop" => Node::Nop,
            "1" => Node::Integer { value: 1 },
            "2" => Node::Integer { value: 2 },
            "" => Node::Empty,
            _ => Node::UnknownIdentifier { name: token },
        };

        if finalize {
            nodes.inner.push(node);
            nodes.last = Node::Empty;
        } else {
            nodes.last = node;
        }
    }

    let mut stack = Stack::new();

    for node in nodes {
        match node {
            Node::Empty => {
                // no effect at runtime
            }

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

            Node::UnknownIdentifier { name: _ } => {
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
