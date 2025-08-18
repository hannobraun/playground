use crate::compiler::tokens::Token;

pub fn compile_input_code(tokens: Vec<Token>) -> Function {
    let mut signature = Signature {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let mut body = Vec::new();

    for identifier in tokens {
        let Token::Identifier { name: identifier } = identifier;

        if let Ok(value) = identifier.parse() {
            body.push(Expression::Value { value });
            signature.outputs.push(Type::I32);
        } else {
            println!("Unknown identifier: `{identifier}`");
            body.push(Expression::Panic);
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
    Panic,
    Value { value: i32 },
}
