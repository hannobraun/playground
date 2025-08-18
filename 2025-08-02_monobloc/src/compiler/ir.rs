pub fn compile_input_code(_: &str) -> Function {
    let signature = Signature {
        inputs: vec![],
        outputs: vec![Type::I32],
    };

    Function {
        signature,
        body: vec![Expression::Value { value: 0 }],
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
