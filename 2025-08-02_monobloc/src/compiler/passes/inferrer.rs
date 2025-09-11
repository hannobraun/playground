use crate::compiler::{
    code::{
        nodes::{Node, NodeKind},
        signatures::Signatures,
        stack::Stack,
    },
    ir::{Intrinsic, Type},
    passes::Resolver,
};

pub struct Inferrer {}

impl Inferrer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_node(
        &mut self,
        node: &Node,
        resolver: &Resolver,
        stack: &mut Stack,
        signatures: &mut Signatures,
    ) {
        process_node(node, stack, signatures, resolver);
    }
}

fn process_node(
    node: &Node,
    stack: &mut Stack,
    signatures: &mut Signatures,
    resolver: &Resolver,
) {
    match &node.kind {
        NodeKind::Binding { names: _ } => {
            for _ in resolver.binding_definitions_at(&node.id) {
                stack.pop(Type::I32);
            }
        }
        NodeKind::Block { block } => {
            let mut stack_for_block = Stack::new();

            for node in &block.nodes {
                process_node(node, &mut stack_for_block, signatures, resolver);
            }

            let signature = stack_for_block.to_signature();
            signatures.insert_and_assign_to_block(node.id, signature.clone());
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
