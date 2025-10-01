//! # StackAssembly: A stack-based, weirdly functional, assembly-like language

#![deny(missing_docs)]

#[cfg(test)]
mod spec;

mod instructions;
mod program;
mod runtime;

pub use self::{program::Program, runtime::Effect};
