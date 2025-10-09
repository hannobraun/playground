use crate::{Effect, Program};

#[test]
fn drop0_drops_operand_at_index_0() {
    let program = Program::compile_and_run("3 5 8 drop0");
    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn popping_from_empty_stack_triggers_effect() {
    let program = Program::compile_and_run("drop0");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::StackUnderflow));
}
