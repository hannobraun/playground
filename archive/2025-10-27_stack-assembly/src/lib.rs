//! # StackAssembly: A stack-based, weirdly functional, assembly-like language

#![deny(missing_docs)]

#[cfg(test)]
mod spec;

mod application;
mod compiler;
mod instructions;
mod runtime;
mod value;

pub use self::{application::Application, instructions::Effect};
