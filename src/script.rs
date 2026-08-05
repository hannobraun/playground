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
        let mut block_is_missing_continuation = true;

        for token in source.split_whitespace() {
            let Some(host_fn) = host.resolve_fn(token) else {
                return Err(CompileError::UnresolvedName);
            };

            block_is_missing_continuation = host.fn_returns(&host_fn);
        }

        if block_is_missing_continuation {
            return Err(CompileError::BlockIsMissingContinuation);
        }

        Ok(Self {})
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
