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
        let ir::Signature { inputs, outputs } = &ir::Signature {
            inputs: vec![],
            outputs: vec![ir::Type::I32],
        };

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
