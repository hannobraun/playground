pub struct Stack {
    pub inner: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, value: i32) {
        self.inner.push(value);
    }
}
