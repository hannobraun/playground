use crate::compiler::{
    inferrer::Inferrer,
    ir::{Block, Expression, Intrinsic, Package},
    nodes::{Node, NodeId, NodeKind},
    resolver::Resolver,
};

pub fn generate(
    nodes: Vec<Node>,
    resolver: &Resolver,
    inferrer: &Inferrer,
) -> Package {
    let mut blocks = Vec::new();
    let root = compile_block(None, nodes, resolver, inferrer, &mut blocks);
    Package { blocks, root }
}

fn compile_block(
    node: Option<NodeId>,
    nodes: Vec<Node>,
    resolver: &Resolver,
    inferrer: &Inferrer,
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
            NodeKind::Block { nodes } => {
                let index = compile_block(
                    Some(node.id),
                    nodes,
                    resolver,
                    inferrer,
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
        .map(|_| unreachable!("`generate_inner` is only called for root block"))
        .unwrap_or_else(|| {
            let signature = inferrer.signature_of_root();
            let bindings = resolver.bindings_for_root().clone();

            (signature, bindings)
        });

    let index = blocks.len();
    let block = Block {
        signature,
        bindings,
        body,
    };

    blocks.push(block);

    index
}
