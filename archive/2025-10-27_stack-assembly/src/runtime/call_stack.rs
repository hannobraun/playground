pub struct CallStack {
    inner: Vec<usize>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, address: usize) {
        self.inner.push(address);
    }

    pub fn pop(&mut self) -> Result<usize, CallStackUnderflow> {
        self.inner.pop().ok_or(CallStackUnderflow)
    }

    pub fn inner(&self) -> &Vec<usize> {
        &self.inner
    }
}

pub struct CallStackUnderflow;
