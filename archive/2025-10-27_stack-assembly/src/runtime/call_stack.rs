use crate::Effect;

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
        address: i32,
    ) -> Result<(), InvalidInstructionAddress> {
        let Ok(address) = address.try_into() else {
            return Err(InvalidInstructionAddress);
        };

        self.advance();
        self.inner.push(address);

        Ok(())
    }

    pub fn pop(&mut self) {
        self.inner.pop();
    }

    pub fn inner(&self) -> &Vec<usize> {
        &self.inner
    }
}

pub struct InvalidInstructionAddress;

impl From<InvalidInstructionAddress> for Effect {
    fn from(InvalidInstructionAddress: InvalidInstructionAddress) -> Self {
        Self::InvalidInstructionAddress
    }
}
