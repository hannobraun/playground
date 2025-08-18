use crate::compiler::wasm::{Emit, leb128::Leb128};

pub struct FuncIdx {
    pub index: u32,
}

impl Emit for FuncIdx {
    fn emit(&self, output: &mut Vec<u8>) {
        Leb128::U32 { value: self.index }.emit(output);
    }
}
