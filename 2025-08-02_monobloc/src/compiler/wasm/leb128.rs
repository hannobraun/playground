use crate::compiler::wasm::Emit;

pub enum Leb128 {
    I32 { value: u32 },
    U32 { value: u32 },
}

impl Emit for Leb128 {
    fn emit(&self, target: &mut Vec<u8>) {
        let result = match *self {
            Self::I32 { value } => {
                let value = i32::from_le_bytes(value.to_le_bytes());
                leb128::write::signed(target, value.into())
            }
            Self::U32 { value } => {
                leb128::write::unsigned(target, value.into())
            }
        };

        result.expect("Writing into a `&mut Vec` should never fail.");
    }
}
