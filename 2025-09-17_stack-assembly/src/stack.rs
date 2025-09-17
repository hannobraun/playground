pub struct Stack {
    pub inner: Vec<i32>,
}

impl Stack {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}
