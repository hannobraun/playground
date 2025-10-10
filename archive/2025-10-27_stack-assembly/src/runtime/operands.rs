use crate::{Effect, value::Value};

pub struct Operands {
    /// # The operand stack
    ///
    /// This uses `i32` instead of `Value`, as this gets exposed to the host,
    /// and the host doesn't have access to `Value` yet.
    inner: Vec<i32>,
}

impl Operands {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.inner.push(value.inner);
    }

    pub fn pop(&mut self) -> Result<Value, StackUnderflow> {
        self.inner
            .pop()
            .map(|value| Value { inner: value })
            .ok_or(StackUnderflow)
    }

    pub fn inner(&mut self) -> &mut Vec<i32> {
        &mut self.inner
    }
}

pub struct StackUnderflow;

impl From<StackUnderflow> for Effect {
    fn from(StackUnderflow: StackUnderflow) -> Self {
        Self::StackUnderflow
    }
}
