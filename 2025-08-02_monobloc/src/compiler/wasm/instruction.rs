use crate::compiler::wasm::{Emit, leb128::Leb128};

pub enum Instruction {
    ConstI32 { value: i32 },
}

impl Emit for Instruction {
    fn emit(&self, target: &mut Vec<u8>) {
        match *self {
            Self::ConstI32 { value } => {
                target.push(0x41);
                Leb128::I32 { value }.emit(target);
            }
        }
    }
}
