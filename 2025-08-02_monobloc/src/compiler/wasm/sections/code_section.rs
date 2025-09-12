use crate::compiler::{
    ir,
    wasm::{
        Emit, expressions::Expressions, leb128::Leb128, sections::Section,
        types::ValType, vec::WasmVec,
    },
};

pub struct CodeSection<'r> {
    pub blocks: &'r [ir::Block],
}

impl Emit for CodeSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        CodeVec {
            blocks: self.blocks,
        }
        .emit(&mut contents);

        Section {
            id: 10,
            contents: &contents,
        }
        .emit(target);
    }
}

struct CodeVec<'r> {
    blocks: &'r [ir::Block],
}

impl Emit for CodeVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let code_entries = self
            .blocks
            .iter()
            .map(|block| Code {
                bindings: &block.bindings,
                body: &block.body,
            })
            .collect::<Vec<_>>();

        WasmVec {
            items: &code_entries,
        }
        .emit(target);
    }
}

struct Code<'r> {
    bindings: &'r [ir::Binding],
    body: &'r ir::Body,
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

struct LocalsVec<'r> {
    bindings: &'r [ir::Binding],
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

struct Locals<'r> {
    n: u32,
    val_type: ValType<'r>,
}

impl Emit for Locals<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        Leb128::U32 { value: self.n }.emit(target);
        self.val_type.emit(target);
    }
}
