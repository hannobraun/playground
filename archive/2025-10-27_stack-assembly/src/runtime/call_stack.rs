pub struct CallStack {
    inner: Vec<usize>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { inner: vec![0] }
    }

    pub fn current_instruction(&mut self) -> Option<&mut usize> {
        self.inner.last_mut()
    }

    pub fn push(&mut self, address: usize) {
        self.inner.push(address);
    }

    pub fn pop(&mut self) {
        self.inner.pop();
    }
}
