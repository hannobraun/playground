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
    let root = generate_inner(None, nodes, resolver, inferrer);
    Package { root }
}

fn generate_inner(
    node: Option<NodeId>,
    nodes: Vec<Node>,
    resolver: &Resolver,
    inferrer: &Inferrer,
) -> Block {
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
            NodeKind::Block { nodes: _ } => {
                // not supported yet; ignoring
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

    Block {
        signature,
        bindings,
        body,
    }
}
