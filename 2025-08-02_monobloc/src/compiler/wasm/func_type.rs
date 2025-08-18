use crate::compiler::wasm::{Emit, val_type::ValType, vec::emit_vec};

pub struct FuncType {}

impl Emit for FuncType {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(0x60);
        ResultType { inner: &[] }.emit(output);
        ResultType { inner: &[] }.emit(output);
    }
}

struct ResultType<'r> {
    inner: &'r [ValType],
}

impl Emit for ResultType<'_> {
    fn emit(&self, output: &mut Vec<u8>) {
        let result_types: &[ValType] = self.inner;
        emit_vec(result_types, output);
    }
}
