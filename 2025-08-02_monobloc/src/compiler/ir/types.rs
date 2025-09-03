#[derive(Clone, Debug)]
pub struct Signature {
    pub inputs: Types,
    pub outputs: Types,
}

pub type Types = Vec<Type>;

#[derive(Clone, Debug)]
pub enum Type {
    Block { signature: Signature },
    I32,
}
