use crate::{
    node::{Node, Nodes},
    stack::{Stack, StackIsEmpty},
};

pub fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut nodes = Nodes::new();

    for token in code.split_whitespace() {
        match token {
            "=" => {
                nodes.inner.push(Node::Equals);
            }
            "assert" => {
                nodes.inner.push(Node::Assert);
            }
            "1" => {
                nodes.inner.push(Node::Integer { value: 1 });
            }
            "2" => {
                nodes.inner.push(Node::Integer { value: 2 });
            }
            unknown => {
                nodes.inner.push(Node::UnknownIdentifier {
                    name: unknown.to_string(),
                });
            }
        }
    }

    let mut stack = Stack::new();

    for node in nodes.inner {
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
