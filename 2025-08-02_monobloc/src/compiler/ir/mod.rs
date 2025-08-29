use crate::compiler::{intrinsics::Intrinsic, ir::types::Signature};

mod generate;
pub mod types;

pub use self::{generate::generate, types::Type};

pub struct Function {
    pub signature: Signature,
    pub body: Body,
}

pub type Body = Vec<Expression>;

#[derive(Clone, Copy)]
pub enum Expression {
    Intrinsic { intrinsic: Intrinsic },
}
