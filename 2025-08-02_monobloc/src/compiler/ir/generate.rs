use crate::compiler::{
    intrinsics::Resolver,
    ir::{
        Expression, Function, Intrinsic,
        types::{Signature, Type, Types},
    },
    syntax::{NodeKind, SyntaxNode},
};

pub fn generate(syntax: Vec<SyntaxNode>, resolver: &Resolver) -> Function {
    let mut stack = Stack {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    let bindings = Vec::new();
    let mut body = Vec::new();

    for node in syntax {
        match node.kind {
            NodeKind::Binding { names: _ } => {
                // Not supported yet; ignoring for now.
            }
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
            NodeKind::Integer { value, format: _ } => {
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
        bindings,
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
