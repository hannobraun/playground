use crate::compiler::{
    ir,
    wasm::{Emit, section::Section, type_idx::TypeIdx, vec::WasmVec},
};

pub struct FunctionSection<'r> {
    pub blocks: &'r [ir::Block],
}

impl Emit for FunctionSection<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let mut contents = Vec::new();
        TypeIdxVec {
            blocks: self.blocks,
        }
        .emit(&mut contents);

        Section {
            id: 3,
            contents: &contents,
        }
        .emit(target);
    }
}

struct TypeIdxVec<'r> {
    blocks: &'r [ir::Block],
}

impl Emit for TypeIdxVec<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        let num_blocks: u32 = self
            .blocks
            .len()
            .try_into()
            .expect("Unsupported number of blocks");

        let type_indices = (0..num_blocks)
            .map(|index| TypeIdx { index })
            .collect::<Vec<_>>();

        WasmVec {
            items: &type_indices,
        }
        .emit(target);
    }
}
