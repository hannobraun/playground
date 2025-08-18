use crate::compiler::wasm::Emit;

pub struct Magic;

impl Emit for Magic {
    fn emit(&self, output: &mut Vec<u8>) {
        output.extend(b"\0asm");
    }
}
