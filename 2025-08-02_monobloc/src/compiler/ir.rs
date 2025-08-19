use crate::compiler::tokens::Token;

pub fn compile_tokens(tokens: Vec<Token>) -> Function {
    let mut stack = Stack {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let mut body = Vec::new();

    for token in tokens {
        match token {
            Token::Comment { text: _ } => {
                // ignoring comment
            }
            Token::Identifier { name } => match name.as_str() {
                "=" => {
                    body.push(Expression::Equals);

                    stack.pop(Type::I32);
                    stack.pop(Type::I32);
                    stack.push(Type::I32);
                }
                "assert" => {
                    body.push(Expression::Assert);
                    stack.pop(Type::I32);
                }
                _ => {
                    println!("Unknown identifier: `{name}`");
                    body.push(Expression::Panic);
                }
            },
            Token::Number { value } => {
                body.push(Expression::Value { value });
                stack.push(Type::I32);
            }
        }
    }

    Function {
        signature: Signature {
            inputs: stack.inputs,
            outputs: stack.outputs,
        },
        body,
    }
}

struct Stack {
    inputs: Types,
    outputs: Types,
}

impl Stack {
    fn push(&mut self, ty: Type) {
        self.outputs.push(ty);
    }

    fn pop(&mut self, ty: Type) {
        if let Some(on_stack) = self.outputs.pop() {
            // We're not checking yet, if the type matches. Since there's only
            // one type so far, it would be redundant anyway.
            let _ = on_stack;
        } else {
            self.inputs.push(ty);
        }
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
    Panic,
    Assert,
    Value { value: i32 },
    Equals,
}
