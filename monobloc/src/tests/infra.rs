//! Infrastructure code used by test suites

use std::collections::BTreeMap;

use crate::{Host, HostFn, host::HostFnAttrs};

pub struct TestHost {
    functions_by_name: BTreeMap<&'static str, u16>,
    attrs: BTreeMap<u16, HostFnAttrs>,
    calls: BTreeMap<u16, usize>,
}

impl TestHost {
    const NAMESPACE: u16 = 256;

    pub fn new<Fn>() -> Self
    where
        Fn: TestHostFn,
    {
        let mut functions_by_name = BTreeMap::new();
        let mut attrs = BTreeMap::new();

        let mut current_id = 0;

        while let Some(function) = Fn::from_id(current_id) {
            functions_by_name.insert(function.name(), current_id);
            attrs.insert(current_id, *function.attrs());

            current_id += 1;
        }

        Self {
            functions_by_name,
            attrs,
            calls: BTreeMap::new(),
        }
    }

    pub fn take_num_calls_to(
        &mut self,
        test_host_fn: impl TestHostFn,
    ) -> usize {
        self.calls.remove(&test_host_fn.id()).unwrap_or(0)
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

    fn fn_attrs(&self, host_fn: &HostFn) -> &HostFnAttrs {
        let Self::NAMESPACE = host_fn.namespace else {
            panic!(
                "Invalid namespace: `{namespace}`",
                namespace = host_fn.namespace,
            );
        };

        &self.attrs[&host_fn.function]
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

pub trait TestHostFn: Into<u16> + TryFrom<u16> {
    fn name(&self) -> &'static str;
    fn attrs(&self) -> &HostFnAttrs;

    fn from_id(id: u16) -> Option<Self> {
        Self::try_from(id).ok()
    }

    fn id(self) -> u16 {
        self.into()
    }
}
