use crate::compiler::wasm::{Emit, leb128::Leb128};

pub struct WasmVec<'a, T> {
    pub items: &'a [T],
}

impl<T> Emit for WasmVec<'_, T>
where
    T: Emit,
{
    fn emit(&self, target: &mut Vec<u8>) {
        Length {
            value: self.items.len(),
        }
        .emit(target);

        for item in self.items {
            item.emit(target);
        }
    }
}

struct Length {
    value: usize,
}

impl Emit for Length {
    fn emit(&self, target: &mut Vec<u8>) {
        let Ok(length) = self.value.try_into() else {
            panic!(
                "Unsupported vector length: `{length}`",
                length = self.value,
            );
        };

        Leb128::U32 { value: length }.emit(target);
    }
}
