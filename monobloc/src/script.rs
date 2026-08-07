use crate::{Host, HostFn};

/// # A compiled Monobloc source code file
///
/// You can create a script from source code by calling [`Script::compile`],
/// then run it using [`Script::run`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Script {
    block: Vec<HostFn>,
}

impl Script {
    /// # Compile the provided source code into a script
    ///
    /// The provided host will be use to resolve host function. Compilation may
    /// produce a [`CompileError`].
    pub fn compile(
        source: &str,
        host: &dyn Host,
    ) -> Result<Self, CompileError> {
        let mut block = Vec::new();
        let mut block_is_missing_continuation = true;

        let mut tokens = source.split_whitespace();

        for token in tokens.by_ref() {
            let Some(host_fn) = host.resolve_fn(token) else {
                return Err(CompileError::UnresolvedName);
            };

            block_is_missing_continuation = host.fn_returns(&host_fn);
            block.push(host_fn);

            if !block_is_missing_continuation {
                break;
            }
        }

        if tokens.next().is_some() {
            return Err(CompileError::UnreachableCode);
        }

        if block_is_missing_continuation {
            return Err(CompileError::BlockIsMissingContinuation);
        }

        Ok(Self { block })
    }

    /// # Run the script to completion
    ///
    /// Host function calls will be relayed to the provided host.
    pub fn run(self, host: &mut dyn Host) {
        for host_fn in self.block {
            host.call_fn(&host_fn);
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

    /// # Code was found in a location that will not be evaluated
    #[error("unreachable code")]
    UnreachableCode,

    /// # A name in the source could not be resolved to a function or binding
    #[error("unresolved name")]
    UnresolvedName,
}
