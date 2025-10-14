use crate::{Effect, Program};

#[test]
fn evaluate_integers() {
    let mut program = Program::compile("3 5");
    assert_eq!(program.operands(), &vec![]);

    program.continue_();
    assert_eq!(program.operands(), &vec![3, 5]);
}

#[test]
fn addition() {
    let mut program = Program::compile_and_run("3 5 +");
    assert_eq!(program.operands(), &vec![8]);
    assert_eq!(program.effect(), None);
}

#[test]
fn addition_triggers_integer_overflow() {
    let mut program = Program::compile_and_run("2147483647 1 +");
    assert_eq!(program.operands(), &vec![]);
    assert_eq!(program.effect(), Some(&Effect::IntegerOverflow));
}
