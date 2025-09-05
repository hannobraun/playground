use crate::compiler::{ir, wasm::module::Module};

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

pub fn generate_module(package: &ir::Package) -> Vec<u8> {
    let mut target = Vec::new();

    Module {
        root: &package.root,
    }
    .emit(&mut target);

    target
}

trait Emit {
    fn emit(&self, target: &mut Vec<u8>);
}

impl Emit for u8 {
    fn emit(&self, target: &mut Vec<u8>) {
        target.push(*self);
    }
}
