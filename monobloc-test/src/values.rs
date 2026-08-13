use arbtest::arbtest;

use monobloc::{CompileError, HostFnAttrs, Script};

use crate::infra::{TestHost, TestHostFn};

#[test]
fn try_to_consume_missing_value() {
    // A lone call to a continuation that consumes a value is invalid, as no
    // value is being provided. This must result in an error.

    let host = TestHost::new::<ValueHostFn>();

    let result = Script::compile("consume exit", &host);
    assert_eq!(result, Err(CompileError::MissingFunctionCallArguments));
}

#[test]
fn produce_and_consume_value() -> anyhow::Result<()> {
    // Producing one value, then consuming it, is valid.

    let mut host = TestHost::new::<ValueHostFn>();

    let script = Script::compile("produce consume exit", &host)?;
    script.run(&mut host);

    Ok(())
}

#[test]
fn leave_value_on_the_stack() {
    // Leaving values on the stack, i.e. producing more than will be consumed,
    // is invalid and must result in an error.

    let host = TestHost::new::<ValueHostFn>();

    let result = Script::compile("produce produce exit", &host);
    assert_eq!(result, Err(CompileError::ValuesLeftOnStack));
}

#[test]
fn stack_overflow() {
    // There's a maximum number of values that the stack can hold. Going over
    // that should result in a compile error.

    let mut source = String::new();

    for _ in 0..256 {
        source.push_str("produce ");
    }

    source.push_str("exit");

    let host = TestHost::new::<ValueHostFn>();

    let result = Script::compile(&source, &host);
    assert_eq!(result, Err(CompileError::StackOverflow));
}

#[test]
fn various_value_related_scenarios() {
    // Compiling the script should always trigger the correct error in regards
    // to values, or compile and run successfully, if there are not too few or
    // too many values.

    arbtest(|u| {
        let mut source = ValueTestSource::new();

        for _ in 0..u.arbitrary_len::<ValueHostFn>()? {
            let value_host_fn = u.arbitrary::<ValueHostFn>()?;
            let true = source.push(value_host_fn) else {
                break;
            };

            if source.num_values < 0 {
                break;
            }
        }

        source.finalize();

        let mut host = TestHost::new::<ValueHostFn>();

        let result = Script::compile(&source.inner, &host);

        match source.num_values {
            i32::MIN..=-1 => {
                assert_eq!(
                    result,
                    Err(CompileError::MissingFunctionCallArguments),
                );
            }
            0 => {
                let script = result.unwrap();
                script.run(&mut host);
            }
            1..=i32::MAX => {
                assert_eq!(result, Err(CompileError::ValuesLeftOnStack),);
            }
        }

        Ok(())
    });
}

#[derive(
    arbitrary::Arbitrary, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
#[repr(u16)]
enum ValueHostFn {
    Consume,
    Exit,
    Produce,
}

impl TestHostFn for ValueHostFn {
    fn attrs(&self) -> &HostFnAttrs {
        match self {
            ValueHostFn::Consume => &HostFnAttrs {
                name: "consume",
                num_parameters: 1,
                return_: Some(0),
            },
            ValueHostFn::Exit => &HostFnAttrs {
                name: "exit",
                num_parameters: 0,
                return_: None,
            },
            ValueHostFn::Produce => &HostFnAttrs {
                name: "produce",
                num_parameters: 0,
                return_: Some(1),
            },
        }
    }
}

struct ValueTestSource {
    inner: String,
    last_call_returns: bool,
    num_values: i32,
}

impl ValueTestSource {
    fn new() -> ValueTestSource {
        Self {
            inner: String::new(),
            last_call_returns: true,
            num_values: 0,
        }
    }

    fn push(&mut self, value_host_fn: ValueHostFn) -> bool {
        if !self.last_call_returns {
            return false;
        }

        if !self.inner.is_empty() {
            self.inner.push(' ');
        }

        match value_host_fn {
            ValueHostFn::Consume => {
                self.num_values -= 1;
            }
            ValueHostFn::Produce => {
                self.num_values += 1;
            }

            ValueHostFn::Exit => {}
        }

        self.inner.push_str(value_host_fn.attrs().name);
        self.last_call_returns = value_host_fn.attrs().return_.is_some();

        true
    }

    fn finalize(&mut self) {
        if self.last_call_returns {
            self.push(ValueHostFn::Exit);
        }
    }
}
