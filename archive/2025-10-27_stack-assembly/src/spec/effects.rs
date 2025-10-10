use crate::{Effect, Program};

#[test]
fn trigger_effect_on_abort() {
    let mut program = Program::compile_and_run("3 abort 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), Some(&Effect::Abort));
}

#[test]
fn unknown_operator_should_trigger_effect() {
    let mut program = Program::compile_and_run("3 unknown_operator 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), Some(&Effect::UnknownOperator));
}

#[test]
fn yield_triggers_effect() {
    let mut program = Program::compile_and_run("3 yield 5");

    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), Some(&Effect::Yield));

    program.continue_();

    assert_eq!(program.operands(), &vec![3, 5]);
    assert_eq!(program.effect(), None);
}
