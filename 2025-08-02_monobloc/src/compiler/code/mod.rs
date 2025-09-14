use crate::compiler::code::{
    intrinsics::Intrinsics, nodes::Nodes, signatures::Signatures, stack::Stack,
};

pub mod bindings;
pub mod intrinsics;
pub mod nodes;
pub mod signatures;
pub mod stack;
pub mod tokens;
pub mod types;

pub struct Code {
    pub nodes: Nodes,
    pub intrinsics: Intrinsics,
    pub stack_for_root: Stack,
    pub signatures: Signatures,
}

impl Code {
    pub fn new() -> Self {
        Self {
            nodes: Nodes::new(),
            intrinsics: Intrinsics::new(),
            stack_for_root: Stack::new(),
            signatures: Signatures::new(),
        }
    }
}
