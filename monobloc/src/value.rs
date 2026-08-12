/// # Representation of a value
///
/// At this point, Monobloc is still an untyped language. All values are
/// represented as 32-bit words.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Value {
    /// The value
    pub bits: u32,
}
