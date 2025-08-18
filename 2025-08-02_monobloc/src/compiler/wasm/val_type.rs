use crate::compiler::wasm::Emit;

pub enum ValType {
    NumType { num_type: NumType },
}

impl Emit for ValType {
    fn emit(&self, output: &mut Vec<u8>) {
        match self {
            ValType::NumType { num_type } => {
                num_type.emit(output);
            }
        }
    }
}

pub enum NumType {
    I32,
}

impl Emit for NumType {
    fn emit(&self, output: &mut Vec<u8>) {
        match self {
            NumType::I32 => {
                output.push(0x7f);
            }
        }
    }
}
