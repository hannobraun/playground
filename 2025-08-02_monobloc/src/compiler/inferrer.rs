use std::collections::BTreeMap;

use crate::compiler::{
    ir::{Intrinsic, Signature, Type, Types},
    nodes::{Node, NodeId, NodeKind},
    resolver::Resolver,
};

pub struct Inferrer {
    stack: Stack,
    signatures: BTreeMap<NodeId, Signature>,
}

impl Inferrer {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            signatures: BTreeMap::new(),
        }
    }

    pub fn process_node(&mut self, node: &Node, resolver: &Resolver) {
        process_node(node, &mut self.stack, &mut self.signatures, resolver);
    }

    pub fn signature_of(&self, node: &NodeId) -> &Signature {
        self.signatures.get(node).expect("Signature not available")
    }

    pub fn signature_of_root(&self) -> Signature {
        self.stack.to_signature()
    }
}

fn process_node(
    node: &Node,
    stack: &mut Stack,
    signatures: &mut BTreeMap<NodeId, Signature>,
    resolver: &Resolver,
) {
    match &node.kind {
        NodeKind::Binding { names: _ } => {
            for _ in resolver.binding_definitions_at(&node.id) {
                stack.pop(Type::I32);
            }
        }
        NodeKind::Block { nodes } => {
            let mut stack_for_block = Stack::new();

            for node in nodes {
                process_node(node, &mut stack_for_block, signatures, resolver);
            }

            let signature = stack_for_block.to_signature();

            signatures.insert(node.id, signature.clone());
            stack.push(Type::Block { signature });
        }
        NodeKind::Comment { text: _ } => {
            // ignoring comment
        }
        NodeKind::Identifier { name: _ } => {
            let intrinsic = resolver.intrinsic_at(&node.id).copied();

            if resolver.binding_call_at(&node.id).is_some() {
                stack.push(Type::I32);
            } else if let Some(Intrinsic::Apply) = intrinsic {
                let Some(Type::Block { signature }) = stack.pop(Type::I32)
                else {
                    panic!("Expected type of `apply` argument to be known.");
                };

                stack.pop(Type::I32);
                for ty in signature.outputs {
                    stack.push(ty);
                }
            } else if let Some([inputs, outputs]) = intrinsic
                .as_ref()
                .and_then(|intrinsic| intrinsic.signature())
            {
                for input in inputs {
                    stack.pop(input.clone());
                }
                for output in outputs {
                    stack.push(output.clone());
                }
            }
        }
        NodeKind::Integer {
            value: _,
            format: _,
        } => {
            stack.push(Type::I32);
        }
    }
}

#[derive(Debug)]
struct Stack {
    inputs: Types,
    outputs: Types,
}

impl Stack {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

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

    fn to_signature(&self) -> Signature {
        Signature {
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
        }
    }
}
