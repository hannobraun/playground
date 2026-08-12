//! # Monobloc, an experimental programming language

mod host;
mod script;

#[cfg(test)]
mod tests;

pub use self::{
    host::{Host, HostFn, HostFnAttrs},
    script::{CompileError, Script},
};
