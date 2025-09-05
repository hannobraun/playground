mod generate;
mod intrinsics;
mod types;

pub use self::{
    generate::generate,
    intrinsics::Intrinsic,
    types::{Signature, Type, Types},
};

pub struct Package {
    pub blocks: Vec<Block>,
}

impl Package {
    pub fn root(&self) -> &Block {
        &self.blocks[0]
    }
}

#[derive(Clone)]
pub struct Block {
    pub signature: Signature,
    pub bindings: Bindings,
    pub body: Body,
}

pub type Bindings = Vec<Binding>;

#[derive(Clone, Debug)]
pub struct Binding {
    pub name: String,
    pub index: u32,
    pub ty: Type,
}

pub type Body = Vec<Expression>;

#[derive(Clone)]
pub enum Expression {
    Bind { index: u32 },
    CallBinding { index: u32 },
    Intrinsic { intrinsic: Intrinsic },
}
