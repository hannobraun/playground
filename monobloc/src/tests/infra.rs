//! Infrastructure code used by test suites

use crate::{Host, HostFn};

#[derive(Default)]
pub struct TestHost {
    pub calls_to_exit: usize,
    pub calls_to_signal: usize,
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
pub enum TestHostFn {
    Exit,
    Signal,
}

impl TestHostFn {
    pub fn returns(&self) -> bool {
        match self {
            TestHostFn::Exit => false,
            TestHostFn::Signal => true,
        }
    }
}
