use crate::{CompileError, Script};

#[test]
fn empty_block() {
    // Every block must call a continuation. Therefore, empty blocks are
    // invalid and must result in a compile error.

    let result = Script::compile("");
    assert_eq!(result, Err(CompileError::BlockIsMissingContinuation));
}
