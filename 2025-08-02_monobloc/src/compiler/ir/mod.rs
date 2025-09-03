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

#[derive(Clone, Debug)]
pub struct Binding {
    pub name: String,
    pub index: u32,
    pub ty: Type,
}

pub type Body = Vec<Expression>;

#[derive(Clone, Copy)]
pub enum Expression {
    Bind { index: u32 },
    CallBinding { index: u32 },
    Intrinsic { intrinsic: Intrinsic },
}
