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
    pub bindings: Bindings,
    pub body: Body,
}

pub type Bindings = Vec<Binding>;

pub struct Binding {
    pub ty: Type,
}

pub type Body = Vec<Expression>;

#[derive(Clone, Copy)]
pub enum Expression {
    Bind { index: u32 },
    Intrinsic { intrinsic: Intrinsic },
}
