use crate::compiler::wasm::{Emit, val_type::ValType, vec::emit_vec};

pub struct FuncType {}

impl Emit for FuncType {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(0x60);
        ResultType.emit(output);
        ResultType.emit(output);
    }
}

struct ResultType;

impl Emit for ResultType {
    fn emit(&self, output: &mut Vec<u8>) {
        let result_types: &[ValType] = &[];
        emit_vec(result_types, output);
    }
}
