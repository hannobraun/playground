pub fn compile_input_code(input_code: &str) -> Function {
    let signature = Signature {
        inputs: vec![],
        outputs: vec![Type::I32],
    };
    let mut body = vec![];

    for identifier in input_code.split_whitespace() {
        if let Ok(value) = identifier.parse() {
            body.push(Expression::Value { value });
        } else {
            println!("Unknown identifier: `{identifier}`");
        }
    }

    Function { signature, body }
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
