use crate::{Effect, Program};

#[test]
fn drop0_drops_operand_at_index_0() {
    let program = Program::compile_and_run("3 5 8 drop0");
    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn drop1_drops_operand_at_index_1() {
    let program = Program::compile_and_run("3 5 8 drop1");
    assert_eq!(program.operands(), &vec![3, 8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn drop2_drops_operand_at_index_2() {
    let program = Program::compile_and_run("3 5 8 drop2");
    assert_eq!(program.operands(), &vec![5, 8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn pick0_picks_operand_at_index_0() {
    let program = Program::compile_and_run("3 5 8 pick0");
    assert_eq!(program.operands(), &vec![3, 5, 8, 8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn pick1_picks_operand_at_index_1() {
    let program = Program::compile_and_run("3 5 8 pick1");
    assert_eq!(program.operands(), &vec![3, 5, 8, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn pick2_picks_operand_at_index_2() {
    let program = Program::compile_and_run("3 5 8 pick2");
    assert_eq!(program.operands(), &vec![3, 5, 8, 3]);
    assert_eq!(program.effect(), None);
}

#[test]
fn popping_from_empty_stack_triggers_effect() {
    let program = Program::compile_and_run("drop0");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::StackUnderflow));
}
