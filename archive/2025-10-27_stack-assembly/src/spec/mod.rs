use crate::Program;

mod control_flow;
mod effects;
mod empty_program;
mod memory;
mod operands;

#[test]
fn evaluate_integers() {
    let mut program = Program::compile("3 5");
    assert_eq!(program.operands(), &vec![]);

    program.continue_();
    assert_eq!(program.operands(), &vec![3, 5]);
}
