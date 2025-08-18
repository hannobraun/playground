use crate::compiler::wasm::{Emit, leb128::Leb128};

pub struct WasmVec<'r, T> {
    pub items: &'r [T],
}

impl<T> Emit for WasmVec<'_, T>
where
    T: Emit,
{
    fn emit(&self, output: &mut Vec<u8>) {
        emit_vec_length(self.items.len(), output);

        for item in self.items {
            item.emit(output);
        }
    }
}

fn emit_vec_length(length: usize, output: &mut Vec<u8>) {
    let Ok(length) = length.try_into() else {
        panic!("Unsupported vector length: `{length}`");
    };

    Leb128::U32 { value: length }.emit(output);
}
