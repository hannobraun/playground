use crate::compiler::{
    code::nodes::{Node, NodeId, NodeKind},
    inferrer::Inferrer,
    ir::{Block, Expression, Intrinsic, Package, Signature},
    resolver::Resolver,
};

pub fn generate(
    nodes: Vec<Node>,
    resolver: &Resolver,
    inferrer: &Inferrer,
) -> Package {
    let mut signatures = Vec::new();
    let mut blocks = Vec::new();

    let root = compile_block(
        None,
        nodes,
        resolver,
        inferrer,
        &mut signatures,
        &mut blocks,
    );

    Package {
        signatures,
        blocks,
        root,
    }
}

fn compile_block(
    node: Option<NodeId>,
    nodes: Vec<Node>,
    resolver: &Resolver,
    inferrer: &Inferrer,
    signatures: &mut Vec<Signature>,
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
                    Some(node.id),
                    block.nodes,
                    resolver,
                    inferrer,
                    signatures,
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

    let (signature, bindings) = node
        .map(|id| {
            let signature = inferrer.signature_of(&id).clone();
            let bindings = resolver.bindings_in(&id).clone();

            (signature, bindings)
        })
        .unwrap_or_else(|| {
            let signature = inferrer.signature_of_root();
            let bindings = resolver.bindings_in_root().clone();

            (signature, bindings)
        });

    let signature = if let Some((index, _)) = signatures
        .iter_mut()
        .enumerate()
        .find(|(_, s)| **s == signature)
    {
        index
    } else {
        let index = signatures.len();
        signatures.push(signature);
        index
    };

    let block = blocks.len();
    blocks.push(Block {
        signature,
        bindings,
        body,
    });

    block
}
