use crate::Effect;

#[derive(Clone, Copy, Debug)]
pub struct Value {
    pub inner: i32,
}

impl Value {
    pub fn into_address(self) -> Result<usize, InvalidAddress> {
        let Ok(address) = self.inner.try_into() else {
            return Err(InvalidAddress);
        };

        Ok(address)
    }
}

pub struct InvalidAddress;

impl From<InvalidAddress> for Effect {
    fn from(InvalidAddress: InvalidAddress) -> Self {
        Self::InvalidOperand
    }
}
