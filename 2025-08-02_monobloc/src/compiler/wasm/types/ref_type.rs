use crate::compiler::wasm::Emit;

pub enum RefType {}

impl Emit for RefType {
    fn emit(&self, _: &mut Vec<u8>) {}
}
