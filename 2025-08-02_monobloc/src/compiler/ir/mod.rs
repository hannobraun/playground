mod generate;

pub use crate::compiler::code::{
    intrinsics::Intrinsic,
    types::{Signature, Type},
};

pub use self::generate::generate;

pub struct Package {
    pub signatures: Vec<Signature>,
    pub blocks: Vec<Block>,
    pub root: usize,
}

impl Package {
    pub fn root(&self) -> &Block {
        &self.blocks[self.root]
    }
}

#[derive(Clone)]
pub struct Block {
    pub signature: usize,
    pub bindings: Vec<Binding>,
    pub body: Body,
}

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
    Block { index: usize },
    CallBinding { index: u32 },
    Intrinsic { intrinsic: Intrinsic },
}
