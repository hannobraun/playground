use crate::{Effect, Program};

mod control_flow;
mod empty_program;
mod operands;
mod yield_;

#[test]
fn stack_stays_empty_if_program_never_starts() {
    let mut program = Program::compile("3");
    assert_eq!(program.operands(), &vec![]);
}

#[test]
fn evaluate_integers() {
    let mut program = Program::compile_and_run("3 5");
    assert_eq!(program.operands(), &vec![3, 5]);
}

#[test]
fn unknown_operator_should_trigger_effect() {
    let mut program = Program::compile_and_run("3 unknown_operator 5");
    assert_eq!(program.operands(), &vec![3]);
    assert_eq!(program.effect(), Some(&Effect::UnknownOperator));
}
