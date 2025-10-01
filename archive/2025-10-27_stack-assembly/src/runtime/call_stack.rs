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

    pub fn push(
        &mut self,
        address: i32,
    ) -> Result<(), InvalidInstructionAddress> {
        let Ok(address) = address.try_into() else {
            return Err(InvalidInstructionAddress);
        };

        if let Some(current_instruction) = self.current_instruction() {
            *current_instruction += 1;
        }

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
