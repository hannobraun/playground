//! Infrastructure code used by test suites

use std::{collections::BTreeMap, fmt};

use crate::{Host, HostFn};

pub struct TestHost {
    functions_by_name: BTreeMap<String, u16>,
    calls: BTreeMap<u16, usize>,
}

impl TestHost {
    const NAMESPACE: u16 = 256;

    pub fn new<Fn>() -> Self
    where
        Fn: TryFrom<u16> + ToString,
    {
        let mut functions_by_name = BTreeMap::new();
        let mut current_id = 0;

        while let Ok(function) = Fn::try_from(current_id) {
            functions_by_name.insert(function.to_string(), current_id);

            current_id += 1;
        }

        Self {
            functions_by_name,
            calls: BTreeMap::new(),
        }
    }

    pub fn take_num_calls_to(&mut self, test_host_fn: impl Into<u16>) -> usize {
        self.calls.remove(&test_host_fn.into()).unwrap_or(0)
    }

    pub fn expect_no_other_calls(&self) {
        assert!(self.calls.is_empty());
    }
}

impl Host for TestHost {
    fn resolve_fn(&self, name: &str) -> Option<HostFn> {
        let &function = self.functions_by_name.get(name)?;

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

        *self.calls.entry(host_fn.function).or_default() += 1;
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

impl fmt::Display for TestHostFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            TestHostFn::Exit => "exit",
            TestHostFn::Signal => "signal",
        };

        write!(f, "{name}")
    }
}
