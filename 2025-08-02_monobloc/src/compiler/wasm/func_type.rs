use crate::compiler::wasm::{
    Emit,
    val_type::{NumType, ValType},
    vec::WasmVec,
};

pub struct FuncType {}

impl Emit for FuncType {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x60);

        let inputs = &[];

        ResultType { inner: inputs }.emit(target);
        ResultType {
            inner: &[ValType::NumType {
                num_type: NumType::I32,
            }],
        }
        .emit(target);
    }
}

struct ResultType<'a> {
    inner: &'a [ValType],
}

impl Emit for ResultType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        WasmVec { items: self.inner }.emit(target);
    }
}
