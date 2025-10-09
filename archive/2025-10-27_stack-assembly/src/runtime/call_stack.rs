use crate::value::{InvalidInstructionAddress, Value};

pub struct CallStack {
    inner: Vec<usize>,
}

impl CallStack {
    pub fn new() -> Self {
        Self { inner: vec![0] }
    }

    pub fn current_instruction(&self) -> Option<usize> {
        self.inner.last().copied()
    }

    pub fn advance(&mut self) {
        if let Some(address) = self.inner.last_mut() {
            *address += 1;
        }
    }

    pub fn push(
        &mut self,
        address: Value,
    ) -> Result<(), InvalidInstructionAddress> {
        let address = address.into_address()?;

        self.advance();
        self.inner.push(address);

        Ok(())
    }

    pub fn pop(&mut self) -> Result<usize, CallStackUnderflow> {
        self.inner.pop().ok_or(CallStackUnderflow)
    }

    pub fn inner(&self) -> &Vec<usize> {
        &self.inner
    }
}

pub struct CallStackUnderflow;
