use crate::Effect;

pub struct Operands {
    inner: Vec<i32>,
}

impl Operands {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.inner.push(value);
    }

    pub fn pop(&mut self) -> Result<i32, StackUnderflow> {
        self.inner.pop().ok_or(StackUnderflow)
    }

    pub fn inner(&self) -> &Vec<i32> {
        &self.inner
    }
}

pub struct StackUnderflow;

impl From<StackUnderflow> for Effect {
    fn from(StackUnderflow: StackUnderflow) -> Self {
        Self::StackUnderflow
    }
}
