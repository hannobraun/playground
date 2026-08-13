use std::iter;

use crate::{Host, HostCall, HostFn, Value};

/// # A compiled Monobloc source code file
///
/// You can create a script from source code by calling [`Script::compile`],
/// then run it using [`Script::run`].
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Script {
    block: Vec<HostFn>,
    stack: Vec<Value>,
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
        let mut num_values: u8 = 0;

        let mut tokens = source.split_whitespace();

        for token in tokens.by_ref() {
            let Some(host_fn) = host.resolve_fn(token) else {
                return Err(CompileError::UnresolvedName);
            };

            let attrs = host.fn_attrs(&host_fn);

            let Some(num_values_after_parameters) =
                num_values.checked_sub(attrs.num_parameters)
            else {
                return Err(CompileError::MissingFunctionCallArguments);
            };
            num_values = num_values_after_parameters;

            if let Some(num_return_params) = attrs.return_ {
                let Some(num_values_after_return) =
                    num_values.checked_add(num_return_params)
                else {
                    return Err(CompileError::StackOverflow);
                };

                num_values = num_values_after_return;
            }

            block_is_missing_continuation = attrs.return_.is_some();
            block.push(host_fn);

            if !block_is_missing_continuation {
                break;
            }
        }

        if num_values > 0 {
            return Err(CompileError::ValuesLeftOnStack);
        }

        if tokens.next().is_some() {
            return Err(CompileError::UnreachableCode);
        }

        if block_is_missing_continuation {
            return Err(CompileError::BlockIsMissingContinuation);
        }

        Ok(Self {
            block,
            stack: Vec::new(),
        })
    }

    /// # Run the script to completion
    ///
    /// Host function calls will be relayed to the provided host.
    pub fn run(mut self, host: &mut dyn Host) {
        for host_fn in self.block {
            let &attrs = host.fn_attrs(&host_fn);

            let mut host_call = ScriptHostCall {
                input: {
                    let num_parameters: usize = attrs.num_parameters.into();

                    self.stack
                        .drain((self.stack.len() - num_parameters)..)
                        .collect()
                },
                output: attrs
                    .return_
                    .into_iter()
                    .flat_map(|num_return_parameters| {
                        let num_return_parameters: usize =
                            num_return_parameters.into();

                        iter::repeat_n(Value { bits: 0 }, num_return_parameters)
                    })
                    .collect(),
            };

            host.call_fn(&host_fn, &mut host_call);

            for value in host_call.output.drain(..) {
                self.stack.push(value);
            }
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

    /// # A function call requires more parameters than arguments are available
    #[error("missing function call arguments")]
    MissingFunctionCallArguments,

    /// # The number of values has exceeded the capacity of the stack
    #[error("stack overflow")]
    StackOverflow,

    /// # Code was found in a location that will not be evaluated
    #[error("unreachable code")]
    UnreachableCode,

    /// # A name in the source could not be resolved to a function or binding
    #[error("unresolved name")]
    UnresolvedName,

    /// # Values are left on the stack at the end of a block
    #[error("values left on stack")]
    ValuesLeftOnStack,
}

struct ScriptHostCall {
    input: Vec<Value>,
    output: Vec<Value>,
}

impl HostCall for ScriptHostCall {
    fn input(&mut self, i: u8) -> Value {
        let i: usize = i.into();
        self.input[i]
    }

    fn output(&mut self, i: u8, value: Value) {
        let i: usize = i.into();
        self.output[i] = value;
    }
}
