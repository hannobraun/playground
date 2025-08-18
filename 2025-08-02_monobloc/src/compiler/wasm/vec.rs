use crate::compiler::wasm::{Emit, leb128::Leb128};

pub struct WasmVec<'r, T> {
    pub items: &'r [T],
}

impl<T> Emit for WasmVec<'_, T>
where
    T: Emit,
{
    fn emit(&self, output: &mut Vec<u8>) {
        Length {
            value: self.items.len(),
        }
        .emit(output);

        for item in self.items {
            item.emit(output);
        }
    }
}

struct Length {
    value: usize,
}

impl Emit for Length {
    fn emit(&self, output: &mut Vec<u8>) {
        let Ok(length) = self.value.try_into() else {
            panic!(
                "Unsupported vector length: `{length}`",
                length = self.value,
            );
        };

        Leb128::U32 { value: length }.emit(output);
    }
}
