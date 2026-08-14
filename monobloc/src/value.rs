/// # Representation of a value
///
/// At this point, Monobloc is still an untyped language. All values are
/// represented as 32-bit words.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Value {
    /// The value
    pub bits: u32,
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value {
            bits: value.to_bits(),
        }
    }
}

impl From<u32> for Value {
    fn from(bits: u32) -> Self {
        Value { bits }
    }
}
