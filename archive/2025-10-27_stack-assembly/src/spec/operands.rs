use crate::{Effect, Program};

#[test]
fn drop0_drops_operand_at_index_0() {
    let mut program = Program::compile_and_run("3 5 8 drop0");
    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn drop1_drops_operand_at_index_1() {
    let mut program = Program::compile_and_run("3 5 8 drop1");
    assert_eq!(program.operands(), &vec![3, 8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn drop2_drops_operand_at_index_2() {
    let mut program = Program::compile_and_run("3 5 8 drop2");
    assert_eq!(program.operands(), &vec![5, 8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn pick0_picks_operand_at_index_0() {
    let mut program = Program::compile_and_run("3 5 8 pick0");
    assert_eq!(program.operands(), &vec![3, 5, 8, 8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn pick1_picks_operand_at_index_1() {
    let mut program = Program::compile_and_run("3 5 8 pick1");
    assert_eq!(program.operands(), &vec![3, 5, 8, 5]);
    assert_eq!(program.effect(), None);
}

#[test]
fn pick2_picks_operand_at_index_2() {
    let mut program = Program::compile_and_run("3 5 8 pick2");
    assert_eq!(program.operands(), &vec![3, 5, 8, 3]);
    assert_eq!(program.effect(), None);
}

#[test]
fn roll2_should_rotate_the_top_2_operands() {
    let mut program = Program::compile_and_run("2 3 5 8 roll2");
    assert_eq!(program.operands(), &vec![2, 3, 8, 5,]);
    assert_eq!(program.effect(), None);
}

#[test]
fn roll3_should_rotate_the_top_3_operands() {
    let mut program = Program::compile_and_run("2 3 5 8 roll3");
    assert_eq!(program.operands(), &vec![2, 5, 8, 3]);
    assert_eq!(program.effect(), None);
}

#[test]
fn roll4_should_rotate_the_top_4_operands() {
    let mut program = Program::compile_and_run("2 3 5 8 roll4");
    assert_eq!(program.operands(), &vec![3, 5, 8, 2]);
    assert_eq!(program.effect(), None);
}

#[test]
fn popping_from_empty_stack_triggers_effect() {
    let mut program = Program::compile_and_run("drop0");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::StackUnderflow));
}
