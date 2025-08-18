use crate::compiler::wasm::Emit;

pub struct Magic;

impl Emit for Magic {
    fn emit(&self, target: &mut Vec<u8>) {
        target.extend(b"\0asm");
    }
}
