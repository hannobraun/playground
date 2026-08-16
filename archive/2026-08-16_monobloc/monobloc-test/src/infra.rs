//! Infrastructure code used by test suites

use std::collections::BTreeMap;

use monobloc::{Host, HostCall, HostFn, HostFnAttrs, Value};

pub struct TestHost {
    functions_by_name: BTreeMap<&'static str, u16>,
    attrs: BTreeMap<u16, HostFnAttrs>,
    calls: BTreeMap<u16, Vec<Vec<Value>>>,
    next_value: u32,
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
            functions_by_name.insert(function.attrs().name, current_id);
            attrs.insert(current_id, *function.attrs());

            current_id += 1;
        }

        Self {
            functions_by_name,
            attrs,
            calls: BTreeMap::new(),
            next_value: 0,
        }
    }

    pub fn take_calls_to(
        &mut self,
        test_host_fn: impl TestHostFn,
    ) -> Vec<Vec<Value>> {
        self.calls.remove(&test_host_fn.id()).unwrap_or_default()
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

    fn call_fn(&mut self, host_fn: &HostFn, host_call: &mut dyn HostCall) {
        let Self::NAMESPACE = host_fn.namespace else {
            panic!(
                "Invalid namespace: `{namespace}`",
                namespace = host_fn.namespace,
            );
        };

        let &attrs = self.fn_attrs(host_fn);

        let mut arguments = Vec::new();

        for i in 0..attrs.num_parameters {
            let value = host_call.input(i);
            arguments.push(value)
        }

        self.calls
            .entry(host_fn.function)
            .or_default()
            .push(arguments);

        if let Some(num_parameters) = attrs.return_ {
            for i in 0..num_parameters {
                let value = Value {
                    bits: self.next_value,
                };
                self.next_value += 1;

                host_call.output(i, value);
            }
        }
    }
}

pub trait TestHostFn: Into<u16> + TryFrom<u16> {
    fn attrs(&self) -> &HostFnAttrs;

    fn from_id(id: u16) -> Option<Self> {
        Self::try_from(id).ok()
    }

    fn id(self) -> u16 {
        self.into()
    }
}
