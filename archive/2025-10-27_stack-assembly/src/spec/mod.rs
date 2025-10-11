use crate::Program;

mod control_flow;
mod effects;
mod empty_program;
mod memory;
mod operands;

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
