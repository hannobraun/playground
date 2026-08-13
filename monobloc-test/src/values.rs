use arbtest::arbtest;

use monobloc::{CompileError, HostFnAttrs, Script};

use crate::infra::{TestHost, TestHostFn};

#[test]
fn try_to_consume_missing_value() {
    // A lone call to a continuation that consumes a value is invalid, as no
    // value is being provided. This must result in an error.

    let host = TestHost::new::<ValueHostFn>();

    let result = Script::compile("exit", &host);
    assert_eq!(result, Err(CompileError::MissingFunctionCallArguments));
}

#[test]
fn produce_and_consume_value() -> anyhow::Result<()> {
    // Producing one value, then consuming it, is valid.

    let mut host = TestHost::new::<ValueHostFn>();

    let script = Script::compile("produce exit", &host)?;
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
fn produce_too_many_values() {
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
        let mut source = String::new();

        let mut last_call_returns = true;
        let mut num_values = 0;

        for _ in 0..u.arbitrary_len::<ValueHostFn>()? {
            if last_call_returns {
                break;
            }
            if !source.is_empty() {
                source.push(' ');
            }

            let value_host_fn = u.arbitrary::<ValueHostFn>()?;
            source.push_str(value_host_fn.attrs().name);

            last_call_returns = value_host_fn.attrs().return_.is_some();

            match value_host_fn {
                ValueHostFn::Consume => {
                    num_values -= 1;

                    if num_values < 0 {
                        break;
                    }
                }
                ValueHostFn::Produce => {
                    num_values += 1;
                }

                ValueHostFn::Exit => {
                    num_values -= 1;
                }
            }
        }

        if last_call_returns {
            num_values -= 1;
            source.push(' ');
            source.push_str(ValueHostFn::Exit.attrs().name);
        }

        let mut host = TestHost::new::<ValueHostFn>();

        let result = Script::compile(&source, &host);

        match num_values {
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
                num_parameters: 1,
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
