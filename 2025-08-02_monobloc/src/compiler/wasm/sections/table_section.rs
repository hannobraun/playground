use crate::compiler::wasm::{sections::Section, types::TableType, vec::WasmVec, Emit};

pub struct TableSection {}

impl Emit for TableSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();

        let items: &[TableType] = &[];
        WasmVec { items }.emit(&mut contents);

        Section {
            id: 4,
            contents: &contents,
        }
        .emit(target);
    }
}
