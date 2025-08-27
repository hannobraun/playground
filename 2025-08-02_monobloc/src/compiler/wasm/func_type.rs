use crate::compiler::{
    types::{Signature, Type, Types},
    wasm::{
        Emit,
        val_type::{NumType, ValType},
        vec::WasmVec,
    },
};

pub struct FuncType<'a> {
    pub signature: &'a Signature,
}

impl Emit for FuncType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let Signature { inputs, outputs } = self.signature;

        target.push(0x60);
        ResultType { types: inputs }.emit(target);
        ResultType { types: outputs }.emit(target);
    }
}

struct ResultType<'a> {
    types: &'a Types,
}

impl Emit for ResultType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let types = compile_types(self.types);
        WasmVec { items: &types }.emit(target);
    }
}

fn compile_types(types: &Types) -> Vec<ValType> {
    types.iter().map(compile_type).collect()
}

fn compile_type(ty: &Type) -> ValType {
    match ty {
        Type::I32 => ValType::NumType {
            num_type: NumType::I32,
        },
    }
}
