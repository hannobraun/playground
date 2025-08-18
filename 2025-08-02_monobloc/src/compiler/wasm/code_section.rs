use crate::compiler::{
    ir,
    wasm::{
        Emit, expressions::Expressions, leb128::Leb128, section::emit_section,
        vec::WasmVec,
    },
};

pub struct CodeSection<'a> {
    pub function: &'a [ir::Expression],
}

impl Emit for CodeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 10;

        let mut contents = Vec::new();
        WasmVec {
            items: &[Code {
                expressions: self.function,
            }],
        }
        .emit(&mut contents);

        emit_section(id, contents, target);
    }
}

struct Code<'a> {
    pub expressions: &'a [ir::Expression],
}

impl Emit for Code<'_> {
    fn emit(&self, output: &mut Vec<u8>) {
        let mut func = Vec::new();
        Locals.emit(&mut func);
        Expressions {
            inner: self.expressions,
        }
        .emit(&mut func);

        let size = func.len();
        let Ok(size) = size.try_into() else {
            panic!("Unsupported code size: `{size}`");
        };

        Leb128::U32 { value: size }.emit(output);
        output.extend(func);
    }
}

struct Locals;

impl Emit for Locals {
    fn emit(&self, output: &mut Vec<u8>) {
        let locals: &[Code] = &[];
        WasmVec { items: locals }.emit(output);
    }
}
