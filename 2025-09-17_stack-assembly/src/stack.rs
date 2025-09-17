pub struct Stack {
    inner: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Result<i32, StackIsEmpty> {
        self.inner.pop().ok_or(StackIsEmpty)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Stack is empty")]
pub struct StackIsEmpty;
