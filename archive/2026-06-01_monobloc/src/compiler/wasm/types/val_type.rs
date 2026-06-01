use crate::compiler::{ir, wasm::Emit};

pub struct ValType<'r> {
    pub ty: &'r ir::Type,
}

impl Emit for ValType<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        match self.ty {
            ir::Type::Block { signature: _ } => NumType::I32.emit(target),
            ir::Type::I32 => NumType::I32.emit(target),
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
