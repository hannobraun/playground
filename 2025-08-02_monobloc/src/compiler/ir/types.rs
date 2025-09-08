#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Signature {
    pub inputs: Vec<Type>,
    pub outputs: Vec<Type>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Block { signature: Signature },
    I32,
}
