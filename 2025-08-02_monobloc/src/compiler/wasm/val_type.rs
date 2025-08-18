use crate::compiler::wasm::Emit;

pub enum ValType {}

impl Emit for ValType {
    fn emit(&self, _: &mut Vec<u8>) {
        match *self {}
    }
}
