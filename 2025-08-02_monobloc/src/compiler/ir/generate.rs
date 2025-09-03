use crate::compiler::{
    ir::{
        Binding, Expression, Function, Intrinsic,
        types::{Signature, Type, Types},
    },
    resolver::Resolver,
    syntax::{NodeKind, SyntaxNode},
};

pub fn generate(syntax: Vec<SyntaxNode>, resolver: &Resolver) -> Function {
    let mut stack = Stack {
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    let mut bindings = Vec::new();
    let mut body = Vec::new();

    for node in syntax {
        match node.kind {
            NodeKind::Binding { names } => {
                for name in names.into_iter().rev() {
                    let index = bindings.len().try_into().expect(
                        "More than `u32::MAX` bindings per scope are not \
                        supported.",
                    );

                    bindings.push(Binding {
                        name,
                        index,
                        ty: Type::I32,
                    });
                    body.push(Expression::Bind { index });
                    stack.pop(Type::I32);
                }
            }
            NodeKind::Comment { text: _ } => {
                // ignoring comment
            }
            NodeKind::Identifier { name } => {
                if let Some(binding) =
                    bindings.iter().rev().find(|binding| binding.name == name)
                {
                    body.push(Expression::CallBinding {
                        index: binding.index,
                    });
                    stack.push(Type::I32);
                } else if let Some(intrinsic) =
                    resolver.intrinsics.get(&node.id).copied()
                {
                    let Some([inputs, outputs]) = intrinsic.signature() else {
                        unreachable!(
                            "Only requesting signature of intrinsics that can \
                            provide it."
                        );
                    };

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
