use crate::stack::{Stack, StackIsEmpty};

pub fn evaluate(code: &str) -> Result<(), EvaluateError> {
    let mut stack = Stack::new();

    for token in code.split_whitespace() {
        match token {
            "=" => {
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
            "assert" => {
                let a = stack.pop()?;

                if a == 0 {
                    return Err(EvaluateError::Other);
                }
            }
            "1" => {
                stack.push(1);
            }
            "2" => {
                stack.push(2);
            }
            _ => {
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
