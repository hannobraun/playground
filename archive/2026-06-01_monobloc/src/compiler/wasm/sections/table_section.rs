use crate::compiler::{
    ir,
    wasm::{
        Emit,
        sections::Section,
        types::{RefType, TableType},
        vec::WasmVec,
    },
};

pub struct TableSection<'r> {
    pub blocks: &'r [ir::Block],
}

impl Emit for TableSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        TableTypeVec {
            blocks: self.blocks,
        }
        .emit(&mut contents);

        Section {
            id: 4,
            contents: &contents,
        }
        .emit(target);
    }
}

struct TableTypeVec<'r> {
    blocks: &'r [ir::Block],
}

impl Emit for TableTypeVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let len = self
            .blocks
            .len()
            .try_into()
            .expect("Unsupported table size");

        let items = &[TableType {
            ref_type: RefType::FuncRef,
            min: len,
            max: len,
        }];

        WasmVec { items }.emit(target);
    }
}
