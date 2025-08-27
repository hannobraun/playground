use crate::compiler::{
    intrinsics::{Intrinsic, Resolver},
    syntax::{SyntaxElementKind, SyntaxNode},
    tokens::Token,
    types::{Signature, Type, Types},
};

pub fn generate_ir(syntax: Vec<SyntaxNode>, resolver: &Resolver) -> Function {
    let mut stack = Stack {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let mut body = Vec::new();

    for syntax_element in syntax {
        match syntax_element.kind {
            SyntaxElementKind::Identifier { name } => {
                if let Some(intrinsic) =
                    resolver.intrinsics.get(&syntax_element.id).copied()
                {
                    let [inputs, outputs] = intrinsic.signature();

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
            SyntaxElementKind::UnprocessedToken {
                token: Token::Comment { text: _ },
            } => {
                // ignoring comment
            }
            SyntaxElementKind::UnprocessedToken {
                token: Token::Identifier { name: _ },
            } => {
                unreachable!(
                    "`Token::Identifier` gets processed by the parser."
                );
            }
            SyntaxElementKind::UnprocessedToken {
                token: Token::IntegerHex { value },
            } => {
                let value = i32::from_le_bytes(value.to_le_bytes());

                body.push(Expression::Intrinsic {
                    intrinsic: Intrinsic::Integer { value },
                });
                stack.push(Type::I32);
            }
            SyntaxElementKind::UnprocessedToken {
                token: Token::IntegerSigned { value },
            } => {
                body.push(Expression::Intrinsic {
                    intrinsic: Intrinsic::Integer { value },
                });
                stack.push(Type::I32);
            }
            SyntaxElementKind::UnprocessedToken {
                token: Token::IntegerUnsigned { value },
            } => {
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
