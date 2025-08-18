use crate::compiler::wasm::Emit;

pub struct Expressions;

impl Emit for Expressions {
    fn emit(&self, output: &mut Vec<u8>) {
        End.emit(output);
    }
}

struct End;

impl Emit for End {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(0x0b);
    }
}
