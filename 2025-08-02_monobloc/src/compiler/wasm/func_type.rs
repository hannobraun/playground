use crate::compiler::{
    ir,
    wasm::{Emit, val_type::ValType, vec::WasmVec},
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
        let types = self
            .types
            .iter()
            .map(|ty| ValType { ty })
            .collect::<Vec<_>>();

        WasmVec { items: &types }.emit(target);
    }
}
