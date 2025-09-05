use crate::compiler::{
    ir,
    wasm::{Emit, func_type::FuncType, section::Section, vec::WasmVec},
};

pub struct TypeSection<'r> {
    pub package: &'r ir::Package,
}

impl Emit for TypeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        FuncTypeVec {
            blocks: &self.package.blocks,
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
