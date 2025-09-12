mod generate;

pub use crate::compiler::code::{
    bindings::Binding, intrinsics::Intrinsic, signatures::Signature,
    types::Type,
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

pub type Body = Vec<Expression>;

#[derive(Clone)]
pub enum Expression {
    Bind { index: u32 },
    Block { index: usize },
    CallBinding { index: u32 },
    Intrinsic { intrinsic: Intrinsic },
}
