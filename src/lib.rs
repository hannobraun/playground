//! # Monobloc, an experimental programming language

mod script;

#[cfg(test)]
mod tests;

pub use self::script::{CompileError, Script};
