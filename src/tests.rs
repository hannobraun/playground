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

#[derive(Default)]
struct TestHost {}

impl Host for TestHost {
    fn resolve_fn(&self, name: &str) -> Option<HostFn> {
        let _ = name;
        None
    }
}
