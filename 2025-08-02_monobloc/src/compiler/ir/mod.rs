use crate::compiler::intrinsics::Intrinsic;

mod generate;
pub mod types;

pub use self::{
    generate::generate,
    types::{Signature, Type, Types},
};

pub struct Function {
    pub signature: Signature,
    pub body: Body,
}

pub type Body = Vec<Expression>;

#[derive(Clone, Copy)]
pub enum Expression {
    Intrinsic { intrinsic: Intrinsic },
}
