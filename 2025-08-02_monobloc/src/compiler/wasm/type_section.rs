use crate::compiler::{
    ir,
    wasm::{Emit, func_type::FuncType, section::Section, vec::WasmVec},
};

pub struct TypeSection<'r> {
    pub blocks: &'r [ir::Block],
}

impl Emit for TypeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        // Here, we emit one type per function. We could reduce code size by
        // deduplicating function types, so we don't need one entry per
        // function.
        let mut contents = Vec::new();
        FuncTypeVec {
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
    blocks: &'r [ir::Block],
}

impl Emit for FuncTypeVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let func_types = self
            .blocks
            .iter()
            .map(|block| FuncType {
                signature: &block.signature,
            })
            .collect::<Vec<_>>();

        WasmVec { items: &func_types }.emit(target);
    }
}
