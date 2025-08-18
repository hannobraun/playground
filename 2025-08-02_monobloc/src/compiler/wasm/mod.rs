use crate::compiler::ir;

mod code_section;
mod export;
mod export_section;
mod expressions;
mod func_idx;
mod func_type;
mod function_section;
mod instruction;
mod leb128;
mod magic;
mod module;
mod section;
mod type_idx;
mod type_section;
mod val_type;
mod vec;
mod version;

pub fn compile_module(_: i32) -> Vec<u8> {
    use crate::compiler::wasm::module::Module;

    let mut target = Vec::new();
    Module {
        function: &[ir::Expression::Value { value: 0 }],
    }
    .emit(&mut target);

    target
}

trait Emit {
    fn emit(&self, output: &mut Vec<u8>);
}

impl Emit for u8 {
    fn emit(&self, output: &mut Vec<u8>) {
        output.push(*self);
    }
}
