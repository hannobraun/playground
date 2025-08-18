use crate::compiler::wasm::{Emit, func_idx::FuncIdx, vec::WasmVec};

pub struct Export<'a> {
    pub name: Name<'a>,
    pub desc: ExportDesc,
}

impl Emit for Export<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        self.name.emit(target);
        self.desc.emit(target);
    }
}

pub struct Name<'a> {
    pub inner: &'a str,
}

impl Emit for Name<'_> {
    fn emit(&self, target: &mut Vec<u8>) {
        WasmVec {
            items: self.inner.as_bytes(),
        }
        .emit(target);
    }
}

pub enum ExportDesc {
    FuncIdx { index: FuncIdx },
}

impl Emit for ExportDesc {
    fn emit(&self, target: &mut Vec<u8>) {
        match self {
            ExportDesc::FuncIdx { index } => {
                target.push(0x00);
                index.emit(target);
            }
        }
    }
}
