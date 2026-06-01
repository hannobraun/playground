use crate::compiler::code::signatures::Signature;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Block { signature: Signature },
    I32,
}
