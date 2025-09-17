use crate::{
    node::Node,
    stack::{Stack, StackIsEmpty},
};

pub fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut nodes = Vec::new();

    for token in code.split_whitespace() {
        match token {
            "=" => {
                nodes.push(Node::Equals);
            }
            "assert" => {
                nodes.push(Node::Assert);
            }
            "1" => {
                nodes.push(Node::Integer { value: 1 });
            }
            "2" => {
                nodes.push(Node::Integer { value: 2 });
            }
            _ => {
                return Err(EvaluateError::Other);
            }
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
            Node::Integer { value } => {
                stack.push(value);
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
