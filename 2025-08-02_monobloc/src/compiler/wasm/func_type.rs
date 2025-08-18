use crate::compiler::wasm::{
    Emit,
    val_type::{NumType, ValType},
    vec::WasmVec,
};

pub struct FuncType {}

impl Emit for FuncType {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(0x60);
        ResultType { inner: &[] }.emit(output);
        ResultType {
            inner: &[ValType::NumType {
                num_type: NumType::I32,
            }],
        }
        .emit(output);
    }
}

struct ResultType<'a> {
    inner: &'a [ValType],
}

impl Emit for ResultType<'_> {
    fn emit(&self, output: &mut Vec<u8>) {
        WasmVec { items: self.inner }.emit(output);
    }
}
