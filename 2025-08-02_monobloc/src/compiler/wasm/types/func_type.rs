use crate::compiler::{
    ir,
    wasm::{Emit, types::ValType, vec::WasmVec},
};

pub struct FuncType<'r> {
    pub signature: &'r ir::Signature,
}

impl Emit for FuncType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let ir::Signature { inputs, outputs } = self.signature;

        target.push(0x60);
        ResultType { types: inputs }.emit(target);
        ResultType { types: outputs }.emit(target);
    }
}

struct ResultType<'r> {
    types: &'r [ir::Type],
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
