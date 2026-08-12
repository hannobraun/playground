//! # Monobloc, an experimental programming language

mod host;
mod script;
mod value;

pub use self::{
    host::{Host, HostFn, HostFnAttrs},
    script::{CompileError, Script},
    value::Value,
};
