use crate::Effect;

#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub inner: i32,
}

impl Value {
    pub fn into_address(self) -> Result<usize, InvalidInstructionAddress> {
        let Ok(address) = self.inner.try_into() else {
            return Err(InvalidInstructionAddress);
        };

        Ok(address)
    }
}

pub struct InvalidInstructionAddress;

impl From<InvalidInstructionAddress> for Effect {
    fn from(InvalidInstructionAddress: InvalidInstructionAddress) -> Self {
        Self::InvalidInstructionAddress
    }
}
