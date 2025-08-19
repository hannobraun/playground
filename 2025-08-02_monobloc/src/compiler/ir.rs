use crate::compiler::tokens::Token;

pub fn compile_input_code(tokens: Vec<Token>) -> Function {
    let mut signature = Signature {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let mut body = Vec::new();

    for token in tokens {
        match token {
            Token::Comment { text: _ } => {
                // ignoring comment
            }
            Token::Identifier { name } => {
                println!("Unknown identifier: `{name}`");
                body.push(Expression::Panic);
            }
            Token::Number { value } => {
                body.push(Expression::Value { value });
                signature.outputs.push(Type::I32);
            }
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
