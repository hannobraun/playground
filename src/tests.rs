use crate::{CompileError, Host, HostFn, Script};

#[test]
fn empty_block() {
    // Every block must call a continuation. Therefore, empty blocks are
    // invalid and must result in a compile error.

    let host = TestHost::default();

    let result = Script::compile("", &host);
    assert_eq!(result, Err(CompileError::BlockIsMissingContinuation));
}

#[test]
fn unresolved_name() {
    // A name in the source that can not be resolved to a function or binding
    // must result in a compile error.

    let host = TestHost::default();

    let result = Script::compile("unresolved", &host);
    assert_eq!(result, Err(CompileError::UnresolvedName));
}

#[test]
fn call_to_function_that_returns() {
    // Calling only a function that returns leaves the caller without a
    // continuation, which must result in an error.

    let host = TestHost::default();

    let result = Script::compile("signal", &host);
    assert_eq!(result, Err(CompileError::BlockIsMissingContinuation));
}

#[test]
fn call_to_function_that_does_not_return() -> anyhow::Result<()> {
    // Calling a function that does not return provides the block with the
    // continuation it must have.

    let host = TestHost::default();

    Script::compile("exit", &host)?;

    Ok(())
}

#[derive(Default)]
struct TestHost {}

impl TestHost {
    const NAMESPACE: u16 = 256;

    const FN_EXIT: u16 = 0;
    const FN_SIGNAL: u16 = 1;
}

impl Host for TestHost {
    fn resolve_fn(&self, name: &str) -> Option<HostFn> {
        let function = match name {
            "exit" => Self::FN_EXIT,
            "signal" => Self::FN_SIGNAL,

            _ => {
                return None;
            }
        };

        Some(HostFn {
            namespace: Self::NAMESPACE,
            function,
        })
    }

    fn fn_returns(&self, host_fn: &HostFn) -> bool {
        let Self::NAMESPACE = host_fn.namespace else {
            panic!(
                "Invalid namespace: `{namespace}`",
                namespace = host_fn.namespace,
            );
        };

        match host_fn.function {
            Self::FN_EXIT => false,
            Self::FN_SIGNAL => true,

            function => {
                panic!("Invalid function: `{function}`");
            }
        }
    }
}
