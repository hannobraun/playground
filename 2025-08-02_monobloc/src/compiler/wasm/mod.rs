use crate::compiler::{ir, wasm::module::Module};

mod export;
mod expressions;
mod func_idx;
mod func_type;
mod instruction;
mod leb128;
mod module;
mod sections;
mod type_idx;
mod type_section;
mod val_type;
mod vec;

pub fn generate_module(package: &ir::Package) -> Vec<u8> {
    let mut target = Vec::new();

    Module { package }.emit(&mut target);

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
