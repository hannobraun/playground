use crate::compiler::{
    ir::{Intrinsic, Signature, Type, Types},
    nodes::{Node, NodeKind},
    resolver::Resolver,
};

/// # Infers the type of a single block
pub struct Inferrer {
    stack: Stack,
}

impl Inferrer {
    pub fn new() -> Self {
        Self {
            stack: Stack {
                inputs: Vec::new(),
                outputs: Vec::new(),
            },
        }
    }

    pub fn process_node(&mut self, node: &Node, resolver: &Resolver) {
        match &node.kind {
            NodeKind::Binding { names: _ } => {
                for _ in resolver.binding_definitions_at(&node.id) {
                    self.stack.pop(Type::I32);
                }
            }
            NodeKind::Block { nodes } => {
                let mut inferrer = Inferrer::new();

                for node in nodes {
                    inferrer.process_node(node, resolver);
                }

                self.stack.push(Type::Block {
                    signature: inferrer.into_signature(),
                });
            }
            NodeKind::Comment { text: _ } => {
                // ignoring comment
            }
            NodeKind::Identifier { name: _ } => {
                let intrinsic = resolver.intrinsic_at(&node.id).copied();

                if resolver.binding_call_at(&node.id).is_some() {
                    self.stack.push(Type::I32);
                } else if let Some(Intrinsic::Apply) = intrinsic {
                    let Some(Type::Block { signature }) =
                        self.stack.pop(Type::I32)
                    else {
                        panic!(
                            "Expected type of `apply` argument to be known."
                        );
                    };

                    self.stack.pop(Type::I32);
                    for ty in signature.outputs {
                        self.stack.push(ty);
                    }
                } else if let Some([inputs, outputs]) = intrinsic
                    .as_ref()
                    .and_then(|intrinsic| intrinsic.signature())
                {
                    for input in inputs {
                        self.stack.pop(input.clone());
                    }
                    for output in outputs {
                        self.stack.push(output.clone());
                    }
                }
            }
            NodeKind::Integer {
                value: _,
                format: _,
            } => {
                self.stack.push(Type::I32);
            }
        }
    }

    /// # Compute the signature of the inferred block
    pub fn into_signature(self) -> Signature {
        Signature {
            inputs: self.stack.inputs,
            outputs: self.stack.outputs,
        }
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

    fn pop(&mut self, expected_type: Type) -> Option<Type> {
        if let Some(type_on_stack) = self.outputs.pop() {
            // We're not checking, if the type on the stack matches the expected
            // type. For the most part, the language is untyped, so values are
            // treated differently, depending on the operation that consumes
            // them.
            //
            // There is a nascent static type system that supports some
            // functions of the language (like generating WebAssembly functions,
            // or figuring out the output of `apply` operations), but overall,
            // it's not complete enough to make a check here sensible.
            Some(type_on_stack)
        } else {
            self.inputs.push(expected_type);
            None
        }
    }
}
