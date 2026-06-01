use crate::compiler::{
    ir,
    wasm::{Emit, sections::Section, types::FuncType, vec::WasmVec},
};

pub struct TypeSection<'r> {
    pub signatures: &'r [ir::Signature],
    pub blocks: &'r [ir::Block],
}

impl Emit for TypeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        // Here, we emit one type per function. We could reduce code size by
        // deduplicating function types, so we don't need one entry per
        // function.
        let mut contents = Vec::new();
        FuncTypeVec {
            signatures: self.signatures,
            blocks: self.blocks,
        }
        .emit(&mut contents);

        Section {
            id: 1,
            contents: &contents,
        }
        .emit(target);
    }
}

struct FuncTypeVec<'r> {
    signatures: &'r [ir::Signature],
    blocks: &'r [ir::Block],
}

impl Emit for FuncTypeVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let func_types = self
            .blocks
            .iter()
            .map(|block| FuncType {
                signature: &self.signatures[block.signature],
            })
            .collect::<Vec<_>>();

        WasmVec { items: &func_types }.emit(target);
    }
}
