use crate::{
    CompileError, Script,
    host::HostFnAttrs,
    tests::infra::{TestHost, TestHostFn},
};

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

#[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u16)]
enum ValueHostFn {
    Exit,
    Produce,
}

impl TestHostFn for ValueHostFn {
    fn attrs(&self) -> &HostFnAttrs {
        match self {
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
