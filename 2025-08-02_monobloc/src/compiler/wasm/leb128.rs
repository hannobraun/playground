use crate::compiler::wasm::Emit;

pub enum Leb128 {
    U32 { value: u32 },
}

impl Emit for Leb128 {
    fn emit(&self, output: &mut Vec<u8>) {
        match *self {
            Leb128::U32 { value } => {
                leb128::write::unsigned(output, value.into())
                    .expect("Writing into a `&mut Vec` should never fail.");
            }
        }
    }
}
