use crate::compiler::wasm::Emit;

pub enum Leb128 {
    I32 { value: i32 },
    U32 { value: u32 },
}

impl Emit for Leb128 {
    fn emit(&self, output: &mut Vec<u8>) {
        let result = match *self {
            Self::I32 { value } => leb128::write::signed(output, value.into()),
            Self::U32 { value } => {
                leb128::write::unsigned(output, value.into())
            }
        };

        result.expect("Writing into a `&mut Vec` should never fail.");
    }
}
