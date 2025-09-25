//! # StackAssembly: A stack-based, weirdly functional, assembly-like language

#![deny(missing_docs)]

#[cfg(test)]
mod spec;

mod program;

pub use self::program::{Effect, Program};
