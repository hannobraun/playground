use crate::compiler::{
    ir::{self},
    wasm::{
        Emit,
        val_type::{NumType, ValType},
        vec::WasmVec,
    },
};

pub struct FuncType<'a> {
    pub signature: &'a ir::Signature,
}

impl Emit for FuncType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let ir::Signature { inputs, outputs } = self.signature;

        target.push(0x60);
        ResultType { types: inputs }.emit(target);
        ResultType { types: outputs }.emit(target);
    }
}

struct ResultType<'a> {
    types: &'a ir::Types,
}

impl Emit for ResultType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let types = compile_types(self.types);
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
