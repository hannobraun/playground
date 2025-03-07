use std::ops;

use crate::range::{self, Range};

#[derive(Clone, Copy, Debug)]
pub struct Value {
    inner: f32,
}

impl Value {
    pub fn new(value: f32) -> Self {
        assert!(
            range::VALUE.contains(value),
            "`Value` must be within the range of [-1, 1]"
        );

        Self { inner: value }
    }

    pub fn encode_from(value: f32, range: Range) -> Self {
        let value = range.convert_value_to(value, range::VALUE);
        Self::new(value)
    }

    pub fn inner(&self) -> f32 {
        self.inner
    }

    pub fn decode_to(&self, range: Range) -> f32 {
        range::VALUE.convert_value_to(self.inner, range)
    }
}

impl ops::Mul<f32> for Value {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let result =
            (self.inner * rhs).clamp(range::VALUE.min, range::VALUE.max);
        Self::new(result)
    }
}
