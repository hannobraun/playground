use arbtest::arbtest;

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

    let mut host = TestHost::default();

    let script = Script::compile("exit", &host)?;
    script.run(&mut host);

    assert_eq!(host.calls_to_exit, 1);
    assert_eq!(host.calls_to_signal, 0);

    Ok(())
}

#[test]
fn unreachable_code() -> anyhow::Result<()> {
    // Any code that follows a call to a function that does not return is
    // invalid, which must result in an error.

    let host = TestHost::default();

    let result = Script::compile("exit signal", &host);
    assert_eq!(result, Err(CompileError::UnreachableCode));

    Ok(())
}

#[test]
fn random_source_code() {
    // Random source code should never trigger a crash.

    arbtest(|u| {
        let source = u.arbitrary::<String>()?;

        let mut host = TestHost::default();

        if let Ok(script) = Script::compile(&source, &host) {
            script.run(&mut host);
        }

        Ok(())
    });
}

#[test]
fn syntactically_correct_source_code() {
    // Syntactically correct source code should never trigger a crash.

    arbtest(|u| {
        let mut source = String::new();

        for _ in 0..u.arbitrary_len::<TestHostFn>()? {
            if !source.is_empty() {
                source.push(' ');
            }

            let fragment = match u.arbitrary::<TestHostFn>()? {
                TestHostFn::Exit => "exit",
                TestHostFn::Signal => "signal",
            };
            source.push_str(fragment);
        }

        let mut host = TestHost::default();

        if let Ok(script) = Script::compile(&source, &host) {
            script.run(&mut host);
        }

        Ok(())
    });
}

#[test]
fn semantically_correct_source_code() {
    // Semantically correct source code should never trigger a crash, always
    // compile, and evaluate all host function calls.

    arbtest(|u| {
        let mut source = String::new();

        let mut calls_to_exit = 0;
        let mut calls_to_signal = 0;

        let mut last_call_returns = true;

        for _ in 0..u.arbitrary_len::<TestHostFn>()? {
            if !source.is_empty() {
                source.push(' ');
            }

            let test_host_fn = u.arbitrary::<TestHostFn>()?;

            let fragment = match test_host_fn {
                TestHostFn::Exit => {
                    calls_to_exit += 1;
                    "exit"
                }
                TestHostFn::Signal => {
                    calls_to_signal += 1;
                    "signal"
                }
            };
            source.push_str(fragment);

            last_call_returns = test_host_fn.returns();
            if !last_call_returns {
                break;
            }
        }

        if last_call_returns {
            calls_to_exit += 1;
            source.push_str(" exit");
        }

        let mut host = TestHost::default();

        let script = Script::compile(&source, &host).unwrap();
        script.run(&mut host);

        assert_eq!(host.calls_to_exit, calls_to_exit);
        assert_eq!(host.calls_to_signal, calls_to_signal);

        Ok(())
    });
}

#[derive(Default)]
struct TestHost {
    calls_to_exit: usize,
    calls_to_signal: usize,
}

impl TestHost {
    const NAMESPACE: u16 = 256;

    const FN_EXIT: u16 = 0;
    const FN_SIGNAL: u16 = 1;
}

impl Host for TestHost {
    fn resolve_fn(&self, name: &str) -> Option<HostFn> {
        let function = match name {
            "exit" => TestHostFn::Exit,
            "signal" => TestHostFn::Signal,

            _ => {
                return None;
            }
        };

        Some(HostFn {
            namespace: Self::NAMESPACE,
            function: function.into(),
        })
    }

    fn fn_returns(&self, host_fn: &HostFn) -> bool {
        let Self::NAMESPACE = host_fn.namespace else {
            panic!(
                "Invalid namespace: `{namespace}`",
                namespace = host_fn.namespace,
            );
        };

        match TestHostFn::try_from(host_fn.function) {
            Ok(test_host_fn) => test_host_fn.returns(),
            Err(function) => {
                panic!("Invalid function: `{function}`");
            }
        }
    }

    fn call_fn(&mut self, host_fn: &HostFn) {
        let Self::NAMESPACE = host_fn.namespace else {
            panic!(
                "Invalid namespace: `{namespace}`",
                namespace = host_fn.namespace,
            );
        };

        match host_fn.function {
            Self::FN_EXIT => {
                self.calls_to_exit += 1;
            }
            Self::FN_SIGNAL => {
                self.calls_to_signal += 1;
            }

            function => {
                panic!("Invalid function: `{function}`");
            }
        }
    }
}

#[derive(
    arbitrary::Arbitrary, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(u16)]
enum TestHostFn {
    Exit,
    Signal,
}

impl TestHostFn {
    fn returns(&self) -> bool {
        match self {
            TestHostFn::Exit => false,
            TestHostFn::Signal => true,
        }
    }
}
