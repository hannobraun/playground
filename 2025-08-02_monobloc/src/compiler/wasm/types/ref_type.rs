use crate::compiler::wasm::Emit;

pub enum RefType {
    FuncRef,
}

impl Emit for RefType {
    fn emit(&self, target: &mut Vec<u8>) {
        match self {
            RefType::FuncRef => target.push(0x70),
        }
    }
}
