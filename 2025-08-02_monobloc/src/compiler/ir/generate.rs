use crate::compiler::{
    code::{
        nodes::{Node, NodeId, NodeKind},
        signatures::Signatures,
        stack::Stack,
    },
    ir::{Block, Expression, Intrinsic, Package},
    passes::{Inferrer, Resolver},
};

pub fn generate(
    nodes: Vec<Node>,
    stack: &Stack,
    signatures: &Signatures,
    resolver: &Resolver,
    _: &Inferrer,
) -> Package {
    let mut blocks = Vec::new();

    let root = compile_block(
        NodeId::root(),
        nodes,
        stack,
        signatures,
        resolver,
        &mut blocks,
    );

    Package {
        signatures: signatures.inner(),
        blocks,
        root,
    }
}

fn compile_block(
    id: NodeId,
    nodes: Vec<Node>,
    stack: &Stack,
    signatures: &Signatures,
    resolver: &Resolver,
    blocks: &mut Vec<Block>,
) -> usize {
    let mut body = Vec::new();

    for node in nodes {
        match node.kind {
            NodeKind::Binding { names: _ } => {
                for binding in resolver.binding_definitions_at(&node.id) {
                    body.push(Expression::Bind {
                        index: binding.index,
                    });
                }
            }
            NodeKind::Block { block } => {
                let index = compile_block(
                    node.id,
                    block.nodes,
                    stack,
                    signatures,
                    resolver,
                    blocks,
                );
                body.push(Expression::Block { index });
            }
            NodeKind::Comment { text: _ } => {
                // ignoring comment
            }
            NodeKind::Identifier { name } => {
                let intrinsic = resolver.intrinsic_at(&node.id).copied();

                if let Some(binding) = resolver.binding_call_at(&node.id) {
                    body.push(Expression::CallBinding {
                        index: binding.index,
                    });
                } else if let Some(intrinsic) = intrinsic {
                    body.push(Expression::Intrinsic { intrinsic });
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
            }
        }
    }

    let (signature, bindings) = if id == NodeId::root() {
        let signature = stack.to_signature();
        let bindings = resolver.bindings_in_root().clone();

        (signature, bindings)
    } else {
        let signature = signatures.get_for_block(&id).clone();
        let bindings = resolver.bindings_in(&id).clone();

        (signature, bindings)
    };

    let signature = signatures
        .index_of(&signature)
        .expect("Expecting signature to be available.");

    let block = blocks.len();
    blocks.push(Block {
        signature,
        bindings,
        body,
    });

    block
}
