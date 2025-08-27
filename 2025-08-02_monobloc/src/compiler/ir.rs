use std::collections::BTreeMap;

use crate::compiler::{
    intrinsics::{Intrinsic, Resolver},
    syntax::{SyntaxElement, SyntaxElementKind},
    tokens::Token,
    types::{Signature, Type, Types},
};

pub fn generate_ir(syntax: Vec<SyntaxElement>) -> Function {
    let resolver = {
        use self::{Intrinsic::*, Type::*};

        let mut intrinsics = BTreeMap::new();
        intrinsics.extend([
            ("%", (Remainder, [&[I32, I32] as &[_], &[I32]])),
            ("*", (Multiply, [&[I32, I32], &[I32]])),
            ("+", (Add, [&[I32, I32], &[I32]])),
            ("-", (Subtract, [&[I32, I32], &[I32]])),
            ("/", (Divide, [&[I32, I32], &[I32]])),
            ("<", (LessThan, [&[I32, I32], &[I32]])),
            ("<=", (LessThanOrEquals, [&[I32, I32], &[I32]])),
            ("=", (Equals, [&[I32, I32], &[I32]])),
            (">", (GreaterThan, [&[I32, I32], &[I32]])),
            (">=", (GreaterThanOrEquals, [&[I32, I32], &[I32]])),
            ("and", (And, [&[I32, I32], &[I32]])),
            ("assert", (Assert, [&[I32], &[]])),
            ("count_ones", (CountOnes, [&[I32], &[I32]])),
            ("leading_zeros", (LeadingZeros, [&[I32], &[I32]])),
            ("not", (Not, [&[I32], &[I32]])),
            ("or", (Or, [&[I32, I32], &[I32]])),
            ("rotate_left", (RotateLeft, [&[I32, I32], &[I32]])),
            ("rotate_right", (RotateRight, [&[I32, I32], &[I32]])),
            ("shift_left", (ShiftLeft, [&[I32, I32], &[I32]])),
            ("shift_right", (ShiftRight, [&[I32, I32], &[I32]])),
            ("trailing_zeros", (TrailingZeros, [&[I32], &[I32]])),
            ("xor", (Xor, [&[I32, I32], &[I32]])),
        ]);

        Resolver { intrinsics }
    };

    let mut stack = Stack {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let mut body = Vec::new();

    for syntax_element in syntax {
        let _ = syntax_element.id;
        let SyntaxElementKind::UnprocessedToken { token } = syntax_element.kind;

        match token {
            Token::Comment { text: _ } => {
                // ignoring comment
            }
            Token::Identifier { name } => {
                if let Some((intrinsic, [inputs, outputs])) =
                    resolver.intrinsics.get(name.as_str()).copied()
                {
                    body.push(Expression::Intrinsic { intrinsic });

                    for &input in inputs {
                        stack.pop(input);
                    }
                    for &output in outputs {
                        stack.push(output);
                    }
                } else {
                    println!("Unknown identifier: `{name}`");
                    body.push(Expression::Intrinsic {
                        intrinsic: Intrinsic::Panic,
                    });
                }
            }
            Token::IntegerHex { value } => {
                let value = i32::from_le_bytes(value.to_le_bytes());

                body.push(Expression::Intrinsic {
                    intrinsic: Intrinsic::Integer { value },
                });
                stack.push(Type::I32);
            }
            Token::IntegerSigned { value } => {
                body.push(Expression::Intrinsic {
                    intrinsic: Intrinsic::Integer { value },
                });
                stack.push(Type::I32);
            }
            Token::IntegerUnsigned { value } => {
                let value = i32::from_le_bytes(value.to_le_bytes());

                body.push(Expression::Intrinsic {
                    intrinsic: Intrinsic::Integer { value },
                });
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

pub type Body = Vec<Expression>;

#[derive(Clone, Copy)]
pub enum Expression {
    Intrinsic { intrinsic: Intrinsic },
}
