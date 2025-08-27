use crate::compiler::{
    intrinsics::{Intrinsic, Resolver},
    syntax::{NodeKind, SyntaxNode},
    tokens::Token,
    types::{Signature, Type, Types},
};

pub fn generate_ir(syntax: Vec<SyntaxNode>, resolver: &Resolver) -> Function {
    let mut stack = Stack {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };
    let mut body = Vec::new();

    for node in syntax {
        match node.kind {
            NodeKind::Comment { text: _ } => {
                // ignoring comment
            }
            NodeKind::Identifier { name } => {
                if let Some(intrinsic) =
                    resolver.intrinsics.get(&node.id).copied()
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
            NodeKind::UnprocessedToken {
                token: Token::Integer { value, format: _ },
            } => {
                body.push(Expression::Intrinsic {
                    intrinsic: Intrinsic::Integer { value },
                });
                stack.push(Type::I32);
            }
            NodeKind::UnprocessedToken { token: _ } => {
                unreachable!("All other tokens get processed by the parser.");
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
