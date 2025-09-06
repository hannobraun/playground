use crate::compiler::wasm::{
    Emit, sections::Section, types::TableType, vec::WasmVec,
};

pub struct TableSection {}

impl Emit for TableSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        TableTypeVec {}.emit(&mut contents);

        Section {
            id: 4,
            contents: &contents,
        }
        .emit(target);
    }
}

struct TableTypeVec {}

impl Emit for TableTypeVec {
    fn emit(&self, target: &mut Vec<u8>) {
        let items: &[TableType] = &[];
        WasmVec { items }.emit(target);
    }
}
