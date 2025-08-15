use crate::compiler::wasm::{Emit, leb128::emit_u32};

pub struct FuncIdx {
    pub index: u32,
}

impl Emit for FuncIdx {
    fn emit(&self, output: &mut Vec<u8>) {
        emit_u32(self.index, output);
    }
}
