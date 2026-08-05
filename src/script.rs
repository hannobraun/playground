use crate::Host;

/// # A compiled Monobloc source code file
///
/// You can create a script from source code by calling [`Script::compile`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Script {}

impl Script {
    /// # Compile the provided source code into a script
    ///
    /// The provided host will be use to resolve host function. Compilation may
    /// produce a [`CompileError`].
    pub fn compile(
        source: &str,
        host: &dyn Host,
    ) -> Result<Self, CompileError> {
        // Placeholder implementation, while the test suite doesn't cover any
        // runtime functionality yet.

        let _ = host;

        if source.is_empty() {
            Err(CompileError::BlockIsMissingContinuation)
        } else {
            Err(CompileError::UnresolvedName)
        }
    }
}

/// # An error that can result from compiling Monobloc source code
///
/// See [`Script::compile`].
#[derive(
    Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, thiserror::Error,
)]
pub enum CompileError {
    /// # A block is missing a continuation
    ///
    /// Every block must call a continuation to define where evaluation should
    /// continue.
    #[error("block is missing a continuation")]
    BlockIsMissingContinuation,

    /// # A name in the source could not be resolved to a function or binding
    #[error("unresolved name")]
    UnresolvedName,
}
