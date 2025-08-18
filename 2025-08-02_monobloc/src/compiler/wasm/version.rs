use crate::compiler::wasm::Emit;

pub struct Version;

impl Emit for Version {
    fn emit(&self, target: &mut Vec<u8>) {
        target.extend([1, 0, 0, 0]);
    }
}
