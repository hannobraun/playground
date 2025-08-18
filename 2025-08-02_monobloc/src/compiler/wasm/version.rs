use crate::compiler::wasm::Emit;

pub struct Version;

impl Emit for Version {
    fn emit(&self, output: &mut Vec<u8>) {
        output.extend([1, 0, 0, 0]);
    }
}
