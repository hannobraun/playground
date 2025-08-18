pub fn compile_input_code(input_code: &str) -> Function {
    let mut signature = Signature {
        inputs: vec![],
        outputs: vec![],
    };
    let mut body = Vec::new();

    for identifier in input_code.split_whitespace() {
        if let Ok(value) = identifier.parse() {
            body.push(Expression::Value { value });
            signature.outputs.push(Type::I32);
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
