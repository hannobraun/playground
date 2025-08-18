pub fn compile_input_code(_: &str) -> Function {
    Function {
        signature: Signature {
            inputs: vec![],
            outputs: vec![Type::I32],
        },
        body: vec![Expression::Value { value: 1 }],
    }
}

pub struct Function {
    pub signature: Signature,
    pub body: Body,
}

pub struct Signature {
    pub inputs: Types,
    pub outputs: Types,
}

pub type Types = Vec<Type>;

pub enum Type {
    I32,
}

pub type Body = Vec<Expression>;

pub enum Expression {
    Value { value: i32 },
}
