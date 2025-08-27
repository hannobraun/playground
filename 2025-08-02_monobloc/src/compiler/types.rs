pub struct Signature {
    pub inputs: Types,
    pub outputs: Types,
}

pub type Types = Vec<Type>;

#[derive(Clone, Copy)]
pub enum Type {
    I32,
}
