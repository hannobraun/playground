use arbtest::arbtest;

use monobloc::{CompileError, HostFnAttrs, Script, Value};

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

    assert_eq!(host.take_calls_to(ValueHostFn::Produce), vec![vec![]]);
    assert_eq!(
        host.take_calls_to(ValueHostFn::Consume),
        vec![vec![Value { bits: 0 }]],
    );
    assert_eq!(host.take_calls_to(ValueHostFn::Exit), vec![vec![]]);

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
fn produce_too_many_values() {
    // Producing more values than get consumed is invalid and must result in a
    // compile error.

    arbtest(|u| {
        let mut source = ValueTestSource::new();

        source.push(ValueHostFn::Produce);

        for _ in 0..u.arbitrary_len::<ValueHostFn>()? {
            let value_host_fn =
                match (u.arbitrary::<ValueHostFn>()?, source.num_values) {
                    (value_host_fn @ ValueHostFn::Consume, n) if n > 2 => {
                        value_host_fn
                    }
                    (value_host_fn @ ValueHostFn::Exit, n) if n > 1 => {
                        value_host_fn
                    }
                    (value_host_fn @ ValueHostFn::Produce, _) => value_host_fn,
                    _ => {
                        continue;
                    }
                };

            let true = source.push(value_host_fn) else {
                break;
            };
        }

        source.finalize();
        assert!(source.num_values > 0);

        let host = TestHost::new::<ValueHostFn>();
        let result = Script::compile(&source.inner, &host);

        assert_eq!(result, Err(CompileError::ValuesLeftOnStack));

        Ok(())
    });
}

#[test]
fn consume_too_many_values() {
    // Consuming more values than get produced is invalid and must result in a
    // compile error.

    arbtest(|u| {
        let mut source = ValueTestSource::new();

        for _ in 0..u.arbitrary_len::<ValueHostFn>()? {
            let value_host_fn =
                match (u.arbitrary::<ValueHostFn>()?, source.num_values) {
                    (ValueHostFn::Exit, n) if n >= 0 => {
                        continue;
                    }
                    (value_host_fn, _) => value_host_fn,
                };

            let true = source.push(value_host_fn) else {
                break;
            };
        }

        // This is relevant if no calls have been generated. Then the call to
        // `finalize` below will add an `exit`, which ends up as a valid script.
        for _ in 0..source.num_values + 1 {
            source.push(ValueHostFn::Consume);
        }

        source.finalize();

        let host = TestHost::new::<ValueHostFn>();
        let result = Script::compile(&source.inner, &host);

        assert_eq!(result, Err(CompileError::MissingFunctionCallArguments));

        Ok(())
    });
}

#[test]
fn balance_production_and_consumption_of_values() {
    // Producing as many values as get consumed is valid.

    arbtest(|u| {
        let mut source = ValueTestSource::new();

        let mut expected_calls_to_produce = Vec::new();
        let mut expected_calls_to_consume = Vec::new();

        let mut expected_values = Vec::new();
        let mut next_expected_value = 0;

        for _ in 0..u.arbitrary_len::<ValueHostFn>()? {
            let value_host_fn =
                match (u.arbitrary::<ValueHostFn>()?, source.num_values) {
                    (ValueHostFn::Consume, n) if n < 1 => {
                        continue;
                    }
                    (ValueHostFn::Exit, n) if n != 0 => {
                        continue;
                    }
                    (value_host_fn, _) => value_host_fn,
                };

            let true = source.push(value_host_fn) else {
                break;
            };

            match value_host_fn {
                ValueHostFn::Consume => {
                    let value = expected_values.pop().unwrap();
                    expected_calls_to_consume.push(vec![value]);
                }
                ValueHostFn::Produce => {
                    expected_values.push(Value {
                        bits: next_expected_value,
                    });
                    next_expected_value += 1;

                    expected_calls_to_produce.push(vec![]);
                }

                ValueHostFn::Exit => {}
            }
        }

        while let Some(value) = expected_values.pop() {
            expected_calls_to_consume.push(vec![value]);
            source.push(ValueHostFn::Consume);
        }

        source.finalize();

        let mut host = TestHost::new::<ValueHostFn>();

        let script = Script::compile(&source.inner, &host).unwrap();
        script.run(&mut host);

        assert_eq!(
            host.take_calls_to(ValueHostFn::Produce),
            expected_calls_to_produce,
        );
        assert_eq!(
            host.take_calls_to(ValueHostFn::Consume),
            expected_calls_to_consume,
        );
        assert_eq!(host.take_calls_to(ValueHostFn::Exit), vec![vec![]]);

        Ok(())
    });
}

#[derive(
    Clone,
    Copy,
    arbitrary::Arbitrary,
    num_enum::IntoPrimitive,
    num_enum::TryFromPrimitive,
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
