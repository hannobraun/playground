use crate::compiler::wasm::{Emit, leb128};

pub struct TypeIdx {
    pub index: u32,
}

impl Emit for TypeIdx {
    fn emit(&self, output: &mut Vec<u8>) {
        leb128::emit_u64(self.index.into(), output);
    }
}
