use crate::Program;

#[test]
fn empty_program_exits_with_empty_stack() {
    let mut program = Program::compile_and_run("");
    assert_eq!(program.operands(), &vec![]);
}

#[test]
fn empty_program_should_not_trigger_effect() {
    let program = Program::compile_and_run("");
    assert_eq!(program.effect(), None);
}
