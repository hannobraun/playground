use crate::compiler::{
    ir,
    wasm::{
        Emit,
        val_type::{NumType, ValType},
        vec::WasmVec,
    },
};

pub struct FuncType {}

impl Emit for FuncType {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x60);

        let inputs = vec![];
        let outputs = vec![ir::Type::I32];

        ResultType { inner: &inputs }.emit(target);
        ResultType { inner: &outputs }.emit(target);
    }
}

struct ResultType<'a> {
    inner: &'a ir::Types,
}

impl Emit for ResultType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let types = compile_types(self.inner);
        WasmVec { items: &types }.emit(target);
    }
}

fn compile_types(types: &ir::Types) -> Vec<ValType> {
    types.iter().map(compile_type).collect()
}

fn compile_type(ty: &ir::Type) -> ValType {
    match ty {
        ir::Type::I32 => ValType::NumType {
            num_type: NumType::I32,
        },
    }
}
