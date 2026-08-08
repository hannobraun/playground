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

#[derive(num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
#[repr(u16)]
enum ValueHostFn {
    Exit,
}

impl TestHostFn for ValueHostFn {
    fn attrs(&self) -> &HostFnAttrs {
        match self {
            ValueHostFn::Exit => &HostFnAttrs {
                name: "exit",
                num_parameters: 1,
                return_: None,
            },
        }
    }
}
