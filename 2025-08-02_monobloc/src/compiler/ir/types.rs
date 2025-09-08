#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Signature {
    pub inputs: Types,
    pub outputs: Types,
}

pub type Types = Vec<Type>;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Block { signature: Signature },
    I32,
}
