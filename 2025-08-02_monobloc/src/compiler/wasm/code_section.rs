use crate::compiler::{
    ir,
    wasm::{
        Emit, expressions::Expressions, leb128::Leb128, section::emit_section,
        val_type::ValType, vec::WasmVec,
    },
};

pub struct CodeSection<'a> {
    pub root: &'a ir::Block,
}

impl Emit for CodeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let id = 10;

        let mut contents = Vec::new();
        WasmVec {
            items: &[Code {
                bindings: &self.root.bindings,
                body: &self.root.body,
            }],
        }
        .emit(&mut contents);

        emit_section(id, &contents, target);
    }
}

struct Code<'a> {
    pub bindings: &'a ir::Bindings,
    pub body: &'a ir::Body,
}

impl Emit for Code<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut func = Vec::new();
        LocalsVec {
            bindings: self.bindings,
        }
        .emit(&mut func);
        Expressions { body: self.body }.emit(&mut func);

        let size = func.len();
        let Ok(size) = size.try_into() else {
            panic!("Unsupported code size: `{size}`");
        };

        Leb128::U32 { value: size }.emit(target);
        target.extend(func);
    }
}

struct LocalsVec<'a> {
    pub bindings: &'a ir::Bindings,
}

impl Emit for LocalsVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let locals = self
            .bindings
            .iter()
            .map(|binding| Locals {
                n: 1,
                val_type: ValType { ty: &binding.ty },
            })
            .collect::<Vec<_>>();

        WasmVec { items: &locals }.emit(target);
    }
}

struct Locals<'a> {
    pub n: u32,
    pub val_type: ValType<'a>,
}

impl Emit for Locals<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        Leb128::U32 { value: self.n }.emit(target);
        self.val_type.emit(target);
    }
}
