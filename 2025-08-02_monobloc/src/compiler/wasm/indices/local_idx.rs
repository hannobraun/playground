use crate::compiler::wasm::{Emit, leb128::Leb128};

#[derive(Clone, Copy)]
pub struct LocalIdx {
    pub index: u32,
}

impl Emit for LocalIdx {
    fn emit(&self, target: &mut Vec<u8>) {
        Leb128::U32 { value: self.index }.emit(target);
    }
}
