use crate::compiler::wasm::{Emit, sections::Section, vec::WasmVec};

pub struct TableSection {}

impl Emit for TableSection {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();

        let items: &[TableSection] = &[];
        WasmVec { items }.emit(&mut contents);

        Section {
            id: 4,
            contents: &contents,
        }
        .emit(target);
    }
}
