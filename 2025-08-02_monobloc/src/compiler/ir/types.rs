pub struct Signature {
    pub inputs: Types,
    pub outputs: Types,
}

pub type Types = Vec<Type>;

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Block,
    I32,
}
