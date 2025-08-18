use crate::compiler::wasm::Emit;

pub enum ValType {
    NumType { num_type: NumType },
}

impl Emit for ValType {
    fn emit(&self, target: &mut Vec<u8>) {
        match self {
            ValType::NumType { num_type } => {
                num_type.emit(target);
            }
        }
    }
}

pub enum NumType {
    I32,
}

impl Emit for NumType {
    fn emit(&self, target: &mut Vec<u8>) {
        match self {
            NumType::I32 => {
                target.push(0x7f);
            }
        }
    }
}
