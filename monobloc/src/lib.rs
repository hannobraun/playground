//! # Monobloc, an experimental programming language

mod host;
mod script;

pub use self::{
    host::{Host, HostFn, HostFnAttrs},
    script::{CompileError, Script},
};
