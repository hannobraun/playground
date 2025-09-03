use crate::compiler::{
    ir::{Signature, Type, Types},
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

                self.stack.push(Type::Block);
            }
            NodeKind::Comment { text: _ } => {
                // ignoring comment
            }
            NodeKind::Identifier { name: _ } => {
                let intrinsic = resolver.intrinsic_at(&node.id).copied();

                if resolver.binding_call_at(&node.id).is_some() {
                    self.stack.push(Type::I32);
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

    fn pop(&mut self, expected_type: Type) {
        if let Some(type_on_stack) = self.outputs.pop() {
            // We're not checking yet, if the type matches. Since there's only
            // one type so far, it would be redundant anyway.
            let _ = type_on_stack;
        } else {
            self.inputs.push(expected_type);
        }
    }
}
