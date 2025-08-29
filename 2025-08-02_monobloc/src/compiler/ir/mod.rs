mod generate;
mod intrinsics;
mod types;

pub use self::{
    generate::generate,
    intrinsics::Intrinsic,
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
