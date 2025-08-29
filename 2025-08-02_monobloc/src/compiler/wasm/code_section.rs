use crate::compiler::{
    ir,
    wasm::{
        Emit, expressions::Expressions, leb128::Leb128, section::emit_section,
        val_type::ValType, vec::WasmVec,
    },
};

pub struct CodeSection<'a> {
    pub root: &'a ir::Body,
}

impl Emit for CodeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 10;

        let mut contents = Vec::new();
        WasmVec {
            items: &[Code { body: self.root }],
        }
        .emit(&mut contents);

        emit_section(id, contents, target);
    }
}

struct Code<'a> {
    pub body: &'a ir::Body,
}

impl Emit for Code<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut func = Vec::new();
        LocalsVec.emit(&mut func);
        Expressions { body: self.body }.emit(&mut func);

        let size = func.len();
        let Ok(size) = size.try_into() else {
            panic!("Unsupported code size: `{size}`");
        };

        Leb128::U32 { value: size }.emit(target);
        target.extend(func);
    }
}

struct LocalsVec;

impl Emit for LocalsVec {
    fn emit(&self, target: &mut Vec<u8>) {
        let locals: [Locals; 0] = [];
        WasmVec { items: &locals }.emit(target);
    }
}

struct Locals {
    pub n: u32,
    pub val_type: ValType,
}

impl Emit for Locals {
    fn emit(&self, target: &mut Vec<u8>) {
        Leb128::U32 { value: self.n }.emit(target);
        self.val_type.emit(target);
    }
}
