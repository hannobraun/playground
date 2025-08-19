use crate::compiler::wasm::{Emit, leb128::Leb128};

pub enum Instruction {
    Unreachable,
    ConstI32 { value: i32 },
    EqI32,
}

impl Emit for Instruction {
    fn emit(&self, target: &mut Vec<u8>) {
        match *self {
            Self::Unreachable => {
                target.push(0x00);
            }
            Self::ConstI32 { value } => {
                target.push(0x41);
                Leb128::I32 { value }.emit(target);
            }
            Self::EqI32 => {
                target.push(0x46);
            }
        }
    }
}

pub struct End;

impl Emit for End {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x0b);
    }
}
