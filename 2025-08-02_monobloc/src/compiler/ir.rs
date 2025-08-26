use std::collections::BTreeMap;

use crate::compiler::tokens::Token;

pub fn compile_tokens(tokens: Vec<Token>) -> Function {
    let intrinsics = {
        use self::{Expression::*, Type::*};

        let mut map = BTreeMap::new();
        map.extend([
            ("*", (Multiply, [&[I32, I32] as &[_], &[I32]])),
            ("+", (Add, [&[I32, I32], &[I32]])),
            ("-", (Subtract, [&[I32, I32], &[I32]])),
            ("/", (Divide, [&[I32, I32], &[I32]])),
            ("<", (LessThan, [&[I32, I32], &[I32]])),
            ("<=", (LessThanOrEquals, [&[I32, I32], &[I32]])),
            ("=", (Equals, [&[I32, I32], &[I32]])),
            (">", (GreaterThan, [&[I32, I32], &[I32]])),
            (">=", (GreaterThanOrEquals, [&[I32, I32], &[I32]])),
            ("assert", (Assert, [&[I32], &[]])),
            ("count_ones", (CountOnes, [&[I32], &[I32]])),
            ("leading_zeros", (LeadingZeros, [&[I32], &[I32]])),
            ("not", (Not, [&[I32], &[I32]])),
            ("trailing_zeros", (TrailingZeros, [&[I32], &[I32]])),
        ]);

        map
    };

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
            Token::Identifier { name } => {
                if let Some((instruction, [inputs, outputs])) =
                    intrinsics.get(name.as_str()).copied()
                {
                    body.push(instruction);

                    for &input in inputs {
                        stack.pop(input);
                    }
                    for &output in outputs {
                        stack.push(output);
                    }
                } else {
                    println!("Unknown identifier: `{name}`");
                    body.push(Expression::Panic);
                }
            }
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

#[derive(Clone, Copy)]
pub enum Type {
    I32,
}

pub type Body = Vec<Expression>;

#[derive(Clone, Copy)]
pub enum Expression {
    Panic,
    Assert,

    Value { value: i32 },

    Equals,
    GreaterThan,
    GreaterThanOrEquals,
    LessThan,
    LessThanOrEquals,
    Not,

    Add,
    Divide,
    Multiply,
    Subtract,

    CountOnes,
    LeadingZeros,
    TrailingZeros,
}
