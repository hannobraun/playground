use crate::compiler::wasm::{Emit, leb128::Leb128, types::RefType};

pub struct TableType {
    pub ref_type: RefType,
    pub min: u32,
    pub max: u32,
}

impl Emit for TableType {
    fn emit(&self, target: &mut Vec<u8>) {
        self.ref_type.emit(target);
        Limits {
            min: self.min,
            max: self.max,
        }
        .emit(target);
    }
}

struct Limits {
    min: u32,
    max: u32,
}

impl Emit for Limits {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(0x01);
        Leb128::U32 { value: self.min }.emit(target);
        Leb128::U32 { value: self.max }.emit(target);
    }
}
